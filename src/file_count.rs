use anyhow::{Error, bail};
use std::convert::TryFrom;
use std::result::Result as StdResult;

#[derive(Clone, Copy, Debug)]
pub struct FileCount(u8);

impl FileCount {
    pub const fn to_u8(self) -> u8 {
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

impl From<FileCount> for u8 {
    fn from(value: FileCount) -> Self {
        value.0
    }
}
