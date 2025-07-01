use crate::dfs::{CatalogueBytes, SECTOR_BYTES};
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum BootOption {
    #[default]
    #[serde(rename = "none")]
    None = 0,
    #[serde(rename = "load")]
    Load = 1,
    #[serde(rename = "run")]
    Run = 2,
    #[serde(rename = "exec")]
    Exec = 3,
}

impl BootOption {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        let temp = bytes[usize::from(SECTOR_BYTES) + 6];
        assert_eq!(0, temp & 0b1100_1100);
        Ok(match (temp & 0b0011_0000) >> 4 {
            0 => Self::None,
            1 => Self::Load,
            2 => Self::Run,
            3 => Self::Exec,
            _ => bail!("invalid boot option {temp}"),
        })
    }

    pub fn write_to_catalogue(self, bytes: &mut [u8]) {
        bytes[usize::from(SECTOR_BYTES) + 6] |= (self as u8) << 4;
    }
}
