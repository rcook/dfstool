use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use crate::util::is_disc_title_char;
use anyhow::{Error, Result, bail};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct DiscTitle(String);

impl DiscTitle {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        let mut title = String::with_capacity(12);
        title.push_str(str::from_utf8(&bytes[0..8])?);
        title.push_str(str::from_utf8(&bytes[SECTOR_SIZE..SECTOR_SIZE + 4])?);
        let s = title.trim_end_matches(' ').trim_end_matches('\0');
        s.parse()
    }

    pub fn write_to(&self, bytes: &mut [u8]) -> Result<()> {
        let temp = self.0.as_bytes();
        bytes[0..temp.len()].copy_from_slice(temp);
        Ok(())
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
        write!(f, "{s}", s = self.0)
    }
}
