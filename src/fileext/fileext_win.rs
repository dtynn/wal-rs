use std::fs::File;
use std::io::Result;
use std::os::windows::FileExt;

pub fn write_at(f: &mut File, buf: &[u8], offset: u64) -> Result<usize> {
    f.seek_write(buf, offset)
}

pub fn read_at(f: &File, buf: &mut [u8], offset: u64) -> Result<usize> {
    f.seek_read(buf, offset)
}
