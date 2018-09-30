#[cfg(unix)]
mod fileext_unix;
#[cfg(unix)]
pub use self::fileext_unix::*;

#[cfg(windows)]
mod fileext_win;
#[cfg(windows)]
pub use self::fileext_win::*;

mod allocate;
pub use self::allocate::allocate;

use std::fs::File;
use std::io::{Error, ErrorKind, Result};

pub fn read_exact_at(f: &File, mut buf: &mut [u8], mut offset: u64) -> Result<()> {
    while !buf.is_empty() {
        match read_at(f, &mut buf, offset) {
            Ok(0) => break,
            Ok(n) => {
                offset += n as u64;
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }

    if !buf.is_empty() {
        return Err(Error::from(ErrorKind::UnexpectedEof));
    }

    Ok(())
}

pub fn write_all_at(f: &File, mut buf: &[u8], mut offset: u64) -> Result<()> {
    while !buf.is_empty() {
        match write_at(f, buf, offset) {
            Ok(0) => {
                return Err(Error::new(
                    ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ))
            }
            Ok(n) => {
                buf = &buf[n..];
                offset += n as u64
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
