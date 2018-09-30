use rand::{thread_rng, Rng};
use std::fs;
use std::path::PathBuf;

pub struct Home(PathBuf);

impl Home {
    pub fn new(p: &str) -> Home {
        let dir = PathBuf::from(String::from(p));
        fs::create_dir_all(&dir).unwrap();
        Home(dir)
    }

    pub fn dir(&self) -> PathBuf {
        self.0.clone()
    }
}

impl Drop for Home {
    fn drop(&mut self) {
        if self.0.is_dir() {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
}

pub fn random_fill(buf: &mut [u8]) {
    thread_rng().fill(buf);
}

pub fn random_bytes(cap: usize) -> Vec<u8> {
    let mut buf = vec![0; cap];

    random_fill(&mut buf);
    buf.to_vec()
}
