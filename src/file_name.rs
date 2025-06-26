use crate::util::is_file_name_char;
use anyhow::{Error, bail};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct FileName(String);

impl FromStr for FileName {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if s.is_empty() || s.len() > 7 || !s.chars().all(is_file_name_char) {
            bail!("invalid file name {s}")
        }

        Ok(Self(String::from(s)))
    }
}

impl Display for FileName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{s}", s = self.0)
    }
}
