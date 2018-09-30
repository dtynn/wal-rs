use config::Config;
use segment::Segment;
use std::ffi::OsStr;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

mod cursor;
use self::cursor::Cursor;

/// WAL write-ahead-log implementation
pub struct WAL {
    cfg: Config,
    dir: PathBuf,
    cursor: Cursor,

    next_sequence: u64,

    segments: Vec<Segment>,
}

impl WAL {
    /// Opens a wal with given dir.
    pub fn open<S: AsRef<OsStr> + ?Sized>(dir: &S, cfg: Config) -> Result<WAL> {
        let p = Path::new(dir);
        if !p.exists() {
            fs::create_dir_all(&p)?;
        }

        if !p.is_dir() {
            return Err(Error::new(ErrorKind::Other, "expecting a directory"));
        }

        let dir = p.to_path_buf();

        let mut cursor = Cursor::open(&dir)?;

        let mut read_sequence = cursor.position.sequence;
        let mut segments: Vec<Segment> = Vec::with_capacity(10);
        loop {
            match Segment::open(&dir, read_sequence, cfg.entry_per_segment, false) {
                Ok(s) => segments.push(s),
                Err(ref e) if e.kind() == ErrorKind::NotFound => break,
                Err(e) => return Err(e),
            }

            read_sequence += 1;
        }

        if segments.is_empty() {
            cursor.position.sequence = 0;
            cursor.position.num = 0;
        };

        Ok(WAL {
            cfg: cfg,
            dir: dir,
            cursor: cursor,
            next_sequence: read_sequence,
            segments: segments,
        })
    }

    /// Writes bytes to wal.
    pub fn write(&mut self, mut data: &[&[u8]]) -> Result<()> {
        while !data.is_empty() {
            let space = self.try_allocate(data.len())?;

            let segment = self.segments.last_mut().unwrap();
            let written = segment.batch_write(&data[0..space])?;
            data = &data[written..];
        }

        Ok(())
    }

    fn try_allocate(&mut self, n: usize) -> Result<(usize)> {
        match self.segments.last_mut() {
            Some(ref s) if s.space() > 0 => {
                let space = s.space();

                return if space > n { Ok(n) } else { Ok(space) };
            }
            Some(s) => {
                let _ = s.flush();
            }
            _ => {}
        }

        let new_seg = Segment::open(
            &self.dir,
            self.next_sequence,
            self.cfg.entry_per_segment,
            true,
        )?;
        let space = new_seg.space();
        self.next_sequence += 1;
        self.segments.push(new_seg);

        if space > n {
            Ok(n)
        } else {
            Ok(space)
        }
    }

    /// Read N entries from wal.
    pub fn read(&mut self, mut n: usize) -> Result<Vec<Vec<u8>>> {
        let mut result: Vec<Vec<u8>> = Vec::with_capacity(n);

        let mut seg_read: usize = 0;
        let start_pos = self.cursor.position.clone();
        let mut start: usize = self.cursor.position.num as usize;

        while n > 0 {
            let segment = match self.segments.get(seg_read) {
                Some(s) if s.len() > start => s,
                _ => break,
            };

            let read = segment.read_into(start, n, &mut result)?;
            start += read;
            n -= read;
            self.cursor.position.sequence = segment.sequence();
            self.cursor.position.num = start as u64;

            if n > 0 {
                start = 0;
                seg_read += 1;
            }
        }

        if seg_read > 1 {
            for _ in 0..seg_read - 1 {
                self.segments.remove(0).destory();
            }
        }

        if self.cursor.position != start_pos {
            let _ = self.cursor.save();
        }

        Ok(result)
    }
}
