use crate::util::is_disc_title_char;
use anyhow::{Error, bail};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct DiscTitle(String);

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
