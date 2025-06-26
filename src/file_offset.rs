use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use crate::file_count::FileCount;
use anyhow::{Result, bail};
use std::convert::From;

#[derive(Debug)]
pub struct FileOffset(u8);

impl FileOffset {
    pub fn number(&self) -> u8 {
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

    pub fn write_to(&self, bytes: &mut [u8]) -> Result<()> {
        bytes[SECTOR_SIZE + 5] = self.0;
        Ok(())
    }

    fn is_in_range(value: u8) -> bool {
        (value & 0b00000111) == 0
    }
}

impl From<FileCount> for FileOffset {
    fn from(value: FileCount) -> Self {
        Self(value.as_u8() << 3)
    }
}
