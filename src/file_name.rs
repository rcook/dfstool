use crate::util::is_file_name_char;
use anyhow::{Error, bail};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
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

impl<'de> Deserialize<'de> for FileName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(SerdeError::custom)
    }
}
