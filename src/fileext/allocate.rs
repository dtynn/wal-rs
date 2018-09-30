use fs2::FileExt;
use std::fs::File;
use std::io::Result;

pub fn allocate(f: &mut File, size: usize) -> Result<()> {
    f.allocate(size as u64)
}
