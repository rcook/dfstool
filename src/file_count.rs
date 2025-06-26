use anyhow::{Error, bail};
use std::convert::TryFrom;
use std::result::Result as StdResult;

pub struct FileCount(u8);

impl FileCount {
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for FileCount {
    type Error = Error;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        if value > 31 {
            bail!("invalid file count {value}")
        }

        Ok(Self(value))
    }
}

impl TryFrom<usize> for FileCount {
    type Error = Error;

    fn try_from(value: usize) -> StdResult<Self, Self::Error> {
        if value > 31 {
            bail!("invalid file count {value}")
        }

        Ok(Self(value as u8))
    }
}
