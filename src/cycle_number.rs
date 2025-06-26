use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use anyhow::{Error, Result, bail};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

#[derive(Debug)]
pub struct CycleNumber(u8);

impl CycleNumber {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        bytes[SECTOR_SIZE + 4].try_into()
    }
}

impl TryFrom<u8> for CycleNumber {
    type Error = Error;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        let hi = value >> 4;
        let lo = value & 0b00001111;
        if hi > 9 || lo > 9 {
            bail!("invalid cycle number {value}")
        }

        Ok(Self(hi * 10 + lo))
    }
}

impl Display for CycleNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{value}", value = self.0)
    }
}
