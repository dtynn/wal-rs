use byteorder::{BigEndian, ByteOrder};
use fileext;
use std::fs::{write, File, OpenOptions};
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

const MAGIC_NUM: [u8; 16] = [
    17, 117, 239, 237, 171, 24, 96, 0, 116, 117, 239, 237, 171, 24, 96, 117,
];

const CURSOR_FILE_NAME: &str = "cursor";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub sequence: u64,
    pub read: u64,
}

pub struct Cursor {
    fname: PathBuf,
    pub position: Position,
}

impl Cursor {
    pub fn open(dir: &PathBuf) -> Result<Cursor> {
        let fname = dir.join(CURSOR_FILE_NAME);
        let file = match OpenOptions::new().read(true).open(&fname) {
            Ok(f) => f,
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                return Ok(Cursor {
                    fname: fname,
                    position: Position {
                        sequence: 0,
                        read: 0,
                    },
                })
            }
            Err(e) => return Err(e),
        };

        let (sequence, read) = read_position(&file)?;

        Ok(Cursor {
            fname: fname,
            position: Position {
                sequence: sequence,
                read: read,
            },
        })
    }

    pub fn save(&mut self) -> Result<()> {
        let mut contents = [0; 32];
        contents[..16].clone_from_slice(&MAGIC_NUM[..]);
        BigEndian::write_u64(&mut contents[16..24], self.position.sequence);
        BigEndian::write_u64(&mut contents[24..32], self.position.read);
        write(&self.fname, contents)
    }
}

fn read_position(f: &File) -> Result<(u64, u64)> {
    let meta = f.metadata()?;
    if meta.len() == 0 {
        return Ok((0, 0));
    }

    let mut buf = [0; 32];
    fileext::read_exact_at(f, &mut buf, 0)?;

    if buf[..16] != MAGIC_NUM {
        return Err(Error::new(ErrorKind::InvalidData, "invalid magic num"));
    }

    Ok((
        BigEndian::read_u64(&buf[16..24]),
        BigEndian::read_u64(&buf[24..32]),
    ))
}
