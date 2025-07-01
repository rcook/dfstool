use crate::dfs::{CatalogueBytes, FileCount, SECTOR_SIZE};
use anyhow::{Result, bail};
use std::convert::From;

#[derive(Debug)]
pub struct FileOffset(u8);

impl FileOffset {
    pub const fn number(&self) -> u8 {
        self.0 >> 3
    }
}

impl FileOffset {
    pub fn new(value: u8) -> Result<Self> {
        if !Self::is_in_range(value) {
            bail!("invalid file offset {value}")
        }

        Ok(Self(value))
    }

    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        let value = bytes[SECTOR_SIZE + 5];
        Self::new(value)
    }

    pub fn write_to_catalogue(&self, bytes: &mut [u8]) {
        bytes[SECTOR_SIZE + 5] = self.0;
    }

    const fn is_in_range(value: u8) -> bool {
        value.trailing_zeros() >= 3
    }
}

impl From<FileCount> for FileOffset {
    fn from(value: FileCount) -> Self {
        Self(u8::from(value) << 3)
    }
}

impl From<FileOffset> for u8 {
    fn from(value: FileOffset) -> Self {
        value.0
    }
}
