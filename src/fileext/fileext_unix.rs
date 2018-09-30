use std::fs::File;
use std::io::Result;
use std::os::unix::fs::FileExt;

pub fn write_at(f: &File, buf: &[u8], offset: u64) -> Result<usize> {
    f.write_at(buf, offset)
}

pub fn read_at(f: &File, buf: &mut [u8], offset: u64) -> Result<usize> {
    f.read_at(buf, offset)
}
