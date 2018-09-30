use super::u64_to_hex;
use super::Segment;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

struct TestHome(PathBuf);

impl TestHome {
    fn new(p: &str) -> TestHome {
        let dir = PathBuf::from(String::from(p));
        fs::create_dir_all(&dir).unwrap();
        TestHome(dir)
    }

    fn dir(&self) -> &PathBuf {
        &self.0
    }
}

impl Drop for TestHome {
    fn drop(&mut self) {
        if self.0.is_dir() {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
}

#[test]
fn test_create_destory() {
    let testhome = TestHome::new("testdir");

    let mut seq = Segment::open(testhome.dir(), 1, 0, true).unwrap();
    let file_base = u64_to_hex(1);
    let fname = Path::new(testhome.dir()).join(file_base.clone() + ".dat");

    assert!(fname.exists() && fname.is_file());

    seq.destory();

    assert!(!fname.exists());
}

#[test]
fn test_read_write() {
    let testhome = TestHome::new("testdir");

    let mut seq = Segment::open(testhome.dir(), 1, 0, true).unwrap();

    let mut buf: [u8; 1024] = [0; 1024];
    thread_rng().fill(&mut buf);

    let mut written: usize = 0;
    while written < buf.len() {
        seq.write(&buf[..written + 1]).unwrap();
        written += 1;
    }

    assert_eq!(seq.len(), buf.len());

    let mut data: Vec<Vec<u8>> = Vec::with_capacity(1024);

    let read = seq.read_into(0, buf.len() + 1, &mut data).unwrap();
    assert_eq!(read, buf.len());
    assert_eq!(data.len(), buf.len());

    for (i, v) in data.iter().enumerate() {
        assert_eq!((i + 1) as usize, v.len());
        assert_eq!(&buf[..(i + 1) as usize].to_vec(), v);
    }

    data.clear();

    let read2 = seq.read_into(255, 100, &mut data).unwrap();
    assert_eq!(read2, 100);
    assert_eq!(data.len(), 100);

    for (i, v) in data.iter().enumerate() {
        assert_eq!((i + 255 + 1) as usize, v.len());
        assert_eq!(&buf[..(i + 255 + 1) as usize].to_vec(), v);
    }
}

#[test]
fn test_write_overlimit() {
    let testhome = TestHome::new("testdir");

    let mut seq = Segment::open(testhome.dir(), 1, 128, true).unwrap();

    let mut buf: [u8; 128] = [0; 128];
    thread_rng().fill(&mut buf);

    let mut written: usize = 0;
    while written < buf.len() {
        seq.write(&buf[..written + 1]).unwrap();
        written += 1;
    }

    assert_eq!(seq.len(), buf.len());

    match seq.write(&buf) {
        Err(ref e) => {
            assert_eq!(e.description(), "entry limit exceeded");
        }
        _ => panic!("expecting error `entry limit exceeded`"),
    }
}
