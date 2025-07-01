use crate::dfs::{CatalogueBytes, SECTOR_SIZE};
use crate::u10;
use anyhow::Result;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

u10!(DiscSize);

impl DiscSize {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        let lo_bits = bytes[SECTOR_SIZE + 7];
        let temp = bytes[SECTOR_SIZE + 6];
        assert_eq!(0, temp & 0b1100_1100);
        let hi_bits = u16::from(temp & 0b0000_0011) << 8;
        let disc_size = hi_bits + u16::from(lo_bits);
        assert!((2..=1023).contains(&disc_size));
        disc_size.try_into()
    }

    pub fn write_to_catalogue(self, bytes: &mut [u8]) {
        let hi = (self.0 >> 8) as u8;
        let lo = (self.0 & 0xff) as u8;
        bytes[SECTOR_SIZE + 7] = lo;
        bytes[SECTOR_SIZE + 6] |= hi;
    }
}

impl Default for DiscSize {
    fn default() -> Self {
        800.try_into().unwrap()
    }
}

impl<'de> Deserialize<'de> for DiscSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u16::deserialize(deserializer)?;
        value.try_into().map_err(SerdeError::custom)
    }
}
