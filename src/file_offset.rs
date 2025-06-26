use anyhow::{Error, bail};
use std::convert::TryFrom;
use std::result::Result as StdResult;

#[derive(Debug)]
pub struct FileOffset(u8);

impl FileOffset {
    pub fn number(&self) -> u8 {
        self.0 >> 3
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
