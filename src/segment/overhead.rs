use byteorder::{BigEndian, ByteOrder};
use std::io;

pub const OVERHEAD_SIZE: usize = 22;
const EMPTY: [u8; OVERHEAD_SIZE] = [0; OVERHEAD_SIZE];

#[derive(Debug)]
pub struct Overhead([u8; OVERHEAD_SIZE]);

impl Overhead {
    pub fn new() -> Overhead {
        let mut a: [u8; OVERHEAD_SIZE] = [0; OVERHEAD_SIZE];
        a[0] = 0x01;
        a[1] = 0xff;
        Overhead(a)
    }

    pub fn write_head(&mut self) {
        self.0[0] = 0x01;
        self.0[1] = 0xff;
    }

    pub fn write_offset(&mut self, offset: u64) {
        BigEndian::write_u64(&mut self.0[2..10], offset);
    }

    pub fn write_size(&mut self, size: u64) {
        BigEndian::write_u64(&mut self.0[10..18], size);
    }

    pub fn write_crc32(&mut self, crc32: u32) {
        BigEndian::write_u32(&mut self.0[18..22], crc32);
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0[..]
    }

    pub fn offset(&self) -> u64 {
        BigEndian::read_u64(&self.0[2..10])
    }

    pub fn size(&self) -> u64 {
        BigEndian::read_u64(&self.0[10..18])
    }

    // pub fn crc32(&self) -> u64 {
    //     BigEndian::read_u64(&self.0[18..22])
    // }

    pub fn valid(&self) -> bool {
        self.0[0] == 0x01 && self.0[1] == 0xff
    }

    pub fn copy_bytes(&mut self, src: &[u8]) -> bool {
        if src.len() < 22 {
            return false;
        }

        self.0[..].copy_from_slice(&src[..22]);
        return true;
    }

    pub fn reset(&mut self) {
        self.copy_bytes(&EMPTY[..]);
    }

    pub fn read_from<T: io::Read>(&mut self, r: &mut T) -> io::Result<()> {
        r.read_exact(&mut self.0[..])
    }
}
