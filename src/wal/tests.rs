use config::Config;
use mock::{random_bytes, Home};
use rand::{thread_rng, Rng};
use segment::Segment;
use std::collections::HashSet;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use wal::WAL;

#[test]
fn test_open_reopen() {
    let per: usize = 100;
    let cfg = Config {
        entry_per_segment: per,
        check_crc32: false,
    };

    let testhome = Home::new("testdir");
    let entry_num: usize = 256;
    let dir = testhome.dir();
    let data = random_bytes(entry_num);
    let buf = data.as_slice();

    {
        let title = "init & write";
        let mut wal = WAL::open(&testhome.dir(), cfg.clone()).unwrap();

        for i in 0..buf.len() {
            wal.write(vec![&buf[..i + 1]].as_slice()).unwrap();
        }

        assert_eq!(wal.segments.len(), 3, "{}", title);
        assert_eq!(wal.len(), entry_num, "{}", title);
        assert_segment_exists(&dir, &vec![0, 1, 2], title);
    }

    {
        let title = "read half in segment 0";
        let mut wal = WAL::open(&testhome.dir(), cfg.clone()).unwrap();

        let read_n = per / 2;
        let out = wal.read(read_n).unwrap();

        assert_eq!(out.len(), read_n, "{}", title);

        assert_eq!(wal.segments.len(), 3, "{}", title);
        assert_eq!(wal.len(), entry_num - read_n, "{}", title);
        assert_segment_exists(&dir, &vec![0, 1, 2], title);
    }

    {
        let title = "read whole segment 0";
        let mut wal = WAL::open(&testhome.dir(), cfg.clone()).unwrap();

        let read_n = per / 2;
        let out = wal.read(read_n).unwrap();

        assert_eq!(out.len(), read_n, "{}", title);

        assert_eq!(wal.segments.len(), 3, "{}", title);
        assert_eq!(wal.len(), entry_num - per, "{}", title);
        assert_segment_exists(&dir, &vec![0, 1, 2], title);
    }

    {
        let title = "read half in segment 1";
        let mut wal = WAL::open(&testhome.dir(), cfg.clone()).unwrap();

        let read_n = per / 2;
        let out = wal.read(read_n).unwrap();

        assert_eq!(out.len(), read_n, "{}", title);

        assert_eq!(wal.segments.len(), 2, "{}", title);
        assert_eq!(wal.len(), entry_num - per - read_n, "{}", title);
        assert_segment_exists(&dir, &vec![1, 2], title);
    }

    {
        let title = "read all";
        let mut wal = WAL::open(&testhome.dir(), cfg.clone()).unwrap();

        let read_n = entry_num - per / 2 * 3;
        let out = wal.read(entry_num * 2).unwrap();

        assert_eq!(out.len(), read_n, "{}", title);

        assert_eq!(wal.segments.len(), 1, "{}", title);
        assert_eq!(wal.len(), 0, "{}", title);
        assert_segment_exists(&dir, &vec![2], title);
    }
}

fn assert_segment_exists<T: Display>(dir: &PathBuf, seqs: &[u64], title: T) {
    for seq in seqs {
        let fname = Segment::filename(*seq);
        let f = Path::new(dir).join(&fname);
        assert!(f.exists() && f.is_file(), "{} {}", title, fname);
    }
}

#[test]
fn test_batch() {
    let per: usize = 10;
    let cfg = Config {
        entry_per_segment: per,
        check_crc32: false,
    };

    let testhome = Home::new("testdir");
    let entry_num: usize = 1024;
    let segment_num = (entry_num + per - 1) / per;
    let dir = testhome.dir();
    let data = random_bytes(entry_num);
    let buf = data.as_slice();

    {
        let title = "batch write";
        let mut wal = WAL::open(&dir, cfg.clone()).unwrap();

        let mut batch: Vec<&[u8]> = Vec::with_capacity(entry_num);

        for i in 0..buf.len() {
            batch.push(&buf[0..i + 1]);
        }

        wal.write(&batch).unwrap();

        assert_eq!(wal.segments.len(), segment_num, "{}", title);
        assert_eq!(wal.len(), entry_num, "{}", title);
    }

    {
        let title = "batch read";
        let mut rng = thread_rng();
        let mut set = HashSet::new();
        let mut left = entry_num;

        while left > 0 {
            let mut wal = WAL::open(&dir, cfg.clone()).unwrap();
            assert_eq!(wal.len(), left, "{}", title);

            let read = if left < per {
                left
            } else {
                rng.gen_range(per / 2, left)
            };

            left -= read;

            let out = wal.read(read).unwrap();
            assert_eq!(out.len(), read);
            assert_eq!(wal.len(), left, "{}", title);

            for one in out {
                assert!(set.insert(one.len()), "{}", title);
            }
        }

        assert_eq!(set.len(), entry_num);
    }
}
