## wal-rs
This is an write-ahead-log implementation.

### Usage
1. Add this lib as a dependency
```
[dependencies]
wal-rs = "1"
```

2. Add the crate reference
```
extern crate wal_rs;
```

### Example
```
extern crate rand;
extern crate wal_rs;

use rand::{thread_rng, Rng};
use std::fs;
use wal_rs::*;

fn main() {
    let cfg = Config {
        entry_per_segment: 100,
        check_crc32: false,
    };

    let mut wal = WAL::open("./testdir", cfg).unwrap();

    let entry_num: usize = 1024;
    let mut buf = vec![0; entry_num];
    thread_rng().fill(&mut buf[..]);

    for i in 0..buf.len() {
        wal.write(&buf[..i + 1]).unwrap();
    }

    assert_eq!(wal.len(), entry_num);

    let data = wal.read(entry_num * 2).unwrap();
    assert_eq!(data.len(), entry_num);
    assert_eq!(wal.len(), 0);
    for (i, v) in data.iter().enumerate() {
        assert_eq!((i + 1) as usize, v.len());
        assert_eq!(&buf[..(i + 1) as usize].to_vec(), v);
    }

    fs::remove_dir_all("./testdir").unwrap();
}

```
