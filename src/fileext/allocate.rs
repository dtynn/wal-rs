use std::fs::File;
use std::io::{copy, Read, Result, Seek, SeekFrom};

#[derive(Debug)]
struct ZeroReader {
    size: usize,
}

impl ZeroReader {
    fn new(size: usize) -> ZeroReader {
        ZeroReader { size: size }
    }
}

impl Read for ZeroReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.size == 0 {
            return Ok(0);
        }

        let mut written: usize = buf.len();
        if written > self.size {
            written = self.size
        }

        self.size -= written;
        Ok(written)
    }
}

pub fn allocate(f: &mut File, size: usize) -> Result<(u64)> {
    f.seek(SeekFrom::Start(0))?;
    let mut reader = ZeroReader::new(size);
    copy(&mut reader, f)
}
