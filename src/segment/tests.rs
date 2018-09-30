use mock::{random_bytes, Home};
use segment::Segment;
use std::path::Path;

#[test]
fn test_create_destory() {
    let testhome = Home::new("testdir");

    let mut seq = Segment::open(&testhome.dir(), 1, 0, true).unwrap();
    let fname = Path::new(&testhome.dir()).join(Segment::filename(1));

    assert!(fname.exists() && fname.is_file());

    seq.destory();

    assert!(!fname.exists());
}

#[test]
fn test_read_write() {
    let testhome = Home::new("testdir");

    let mut seq = Segment::open(&testhome.dir(), 1, 0, true).unwrap();

    let buf_vec = random_bytes(1024);
    let buf = buf_vec.as_slice();

    let mut written: usize = 0;
    while written < buf.len() {
        seq.write(&buf[..written + 1]).unwrap();
        written += 1;
    }

    assert_eq!(seq.len(), buf.len());

    let mut data: Vec<Vec<u8>> = Vec::with_capacity(1024);

    let read = seq.read_into(0, buf.len() + 1, &mut data, true).unwrap();
    assert_eq!(read, buf.len());
    assert_eq!(data.len(), buf.len());

    for (i, v) in data.iter().enumerate() {
        assert_eq!((i + 1) as usize, v.len());
        assert_eq!(&buf[..(i + 1) as usize].to_vec(), v);
    }

    data.clear();

    let read2 = seq.read_into(255, 100, &mut data, true).unwrap();
    assert_eq!(read2, 100);
    assert_eq!(data.len(), 100);

    for (i, v) in data.iter().enumerate() {
        assert_eq!((i + 255 + 1) as usize, v.len());
        assert_eq!(&buf[..(i + 255 + 1) as usize].to_vec(), v);
    }
}

#[test]
fn test_write_overlimit() {
    let testhome = Home::new("testdir");

    let mut seq = Segment::open(&testhome.dir(), 1, 128, true).unwrap();

    let buf_vec = random_bytes(128);
    let buf = buf_vec.as_slice();

    let mut written: usize = 0;
    while written < buf.len() {
        seq.write(&buf[..written + 1]).unwrap();
        written += 1;
    }

    assert_eq!(seq.len(), buf.len());

    match seq.write(&buf) {
        Ok(false) => {}
        _ => panic!("expecting error `entry limit exceeded`"),
    }
}
