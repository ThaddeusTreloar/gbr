pub mod mbc1;

use crate::error::{MbcReadError, MbcWriteError};

pub struct Enabled;
pub struct Disabled;

pub struct ReadMode;
pub struct WriteMode;

pub trait MbcRead {
    fn read_byte(address: u16) -> Result<u32, MbcReadError>;
}

pub trait MbcWrite {
    fn write_byte(address: u16, payload: u32) -> Result<u32, MbcWriteError>;
}