use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use anyhow::{Result, bail};
use std::convert::TryFrom;
use std::result::Result as StdResult;

#[derive(Debug)]
#[repr(u8)]
pub enum BootOption {
    None = 0,
    Load = 1,
    Run = 2,
    Exec = 3,
}

impl BootOption {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        let temp = bytes[SECTOR_SIZE + 6];
        assert_eq!(0, temp & 0b11001100);
        temp.try_into()
    }
}

impl TryFrom<u8> for BootOption {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        Ok(match (value & 0b00110000) >> 4 {
            0 => Self::None,
            1 => Self::Load,
            2 => Self::Run,
            3 => Self::Exec,
            _ => bail!("invalid boot option {value}"),
        })
    }
}
