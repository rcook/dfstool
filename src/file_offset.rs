use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use anyhow::{Error, Result, bail};
use std::convert::TryFrom;
use std::result::Result as StdResult;

#[derive(Debug)]
pub struct FileOffset(u8);

impl FileOffset {
    pub fn number(&self) -> u8 {
        self.0 >> 3
    }
}

impl FileOffset {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        let offset = bytes[SECTOR_SIZE + 5];
        offset.try_into()
    }
}

impl TryFrom<u8> for FileOffset {
    type Error = Error;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        if (value & 0b00000111) != 0 {
            bail!("invalid file offset {value}")
        }

        Ok(Self(value))
    }
}
