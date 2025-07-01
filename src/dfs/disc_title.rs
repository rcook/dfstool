use crate::dfs::{CatalogueBytes, SECTOR_SIZE};
use crate::util::is_disc_title_char;
use anyhow::{Error, Result, bail};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, Serialize)]
pub struct DiscTitle(String);

impl DiscTitle {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        let mut title = String::with_capacity(12);
        title.push_str(str::from_utf8(&bytes[0..8])?);
        title.push_str(str::from_utf8(&bytes[SECTOR_SIZE..SECTOR_SIZE + 4])?);
        let s = title.trim_end_matches(' ').trim_end_matches('\0');
        s.parse()
    }

    pub fn write_to_catalogue(&self, bytes: &mut [u8]) {
        let temp = self.0.as_bytes();
        bytes[0..temp.len()].copy_from_slice(temp);
    }
}

impl FromStr for DiscTitle {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if s.len() > 12 || !s.chars().all(is_disc_title_char) {
            bail!("invalid disc title {s}")
        }

        Ok(Self(String::from(s)))
    }
}

impl Display for DiscTitle {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl<'de> Deserialize<'de> for DiscTitle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(SerdeError::custom)
    }
}
