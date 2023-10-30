use std::marker::PhantomData;

use crate::error::{MbcReadError, MbcWriteError};

use super::{Enabled, ReadMode, MbcRead, MbcWrite, WriteMode};

const ROM_BANK_SIZE: u16 = 0x4000;
const RAM_BANK_SIZE: u16 = 0x2000;

const ROM_BANK_START: u16 = 0x4000;
const ROM_BANK_END: u16 = 0x7FFF;

const RAM_BANK_START: u16 = 0xA000;
const RAM_BANK_END: u16 = 0xBFFF;

pub struct MbcOne<T, M> {
    pub rom: [u8; 0x4000],
    pub ram: [u8; 0xB8000],
    pub rom_bank_low: u8,
    pub ram_bank_upper: u8,
    pub ram_enabled: PhantomData<T>,
    pub access_mode: PhantomData<M>,
}

impl MbcRead for MbcOne<Enabled, ReadMode> {

    fn read_byte(addr: u16) -> Result<u32, MbcReadError> {
        match addr {
            addr if addr < ROM_BANK_START => {
                // ROM bank 0
                unimplemented!()
            }
            addr if ROM_BANK_START < addr && addr <= ROM_BANK_END => {
                // ROM bank 1 - 0x3FFF - 0x7FFF
                unimplemented!()
            }
            addr if RAM_BANK_START < addr && addr <= RAM_BANK_END => {
                // VRAM - 0x8000 - 0x9FFF
                unimplemented!()
            }
            _ => Err(MbcReadError::ReadAddressOutOfRange(addr as u32)),
        }
    }
}
impl MbcWrite for MbcOne<Enabled, WriteMode> {
    fn write_byte(address: u16, payload: u32) -> Result<u32, MbcWriteError> {
        match address {
            address if address < 0x1FFF => {
                // RAM enable/disable - 0x0000 - 0x1FFF
                unimplemented!()
            }
            address if address < 0x3FFF => {
                // ROM bank number - 0x2000 - 0x3FFF
                unimplemented!()
            }
            address if address < 0x5FFF => {
                // RAM bank number - 0x4000 - 0x5FFF
                unimplemented!()
            }
            address if address < 0x7FFF => {
                // ROM/RAM mode select - 0x6000 - 0x7FFF
                unimplemented!()
            }
            address if address < 0xA000 => {
                // VRAM - 0x8000 - 0x9FFF
                unimplemented!()
            }
            _ => Err(MbcWriteError::WriteAddressOutOfRange(address as u32)),
        }
    }
}
