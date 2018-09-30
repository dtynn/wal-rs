//! This crate is implementation of an on-disk write-ahead-log
//!

#![warn(missing_docs)]

extern crate byteorder;
extern crate crc;
extern crate fs2;
extern crate hex;

#[cfg(test)]
extern crate rand;

mod config;
mod fileext;
mod segment;
mod wal;

#[cfg(test)]
mod mock;

pub use config::Config;
pub use wal::WAL;
