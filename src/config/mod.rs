/// WAL config
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Config {
    /// entry limit of a single segment file
    pub entry_per_segment: usize,

    /// if we should do check_sum
    pub check_crc32: bool,
}
