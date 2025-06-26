use crate::util::is_file_name_char;
use anyhow::{Error, bail};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Directory(char);

impl Directory {
    pub fn as_char(&self) -> char {
        self.0
    }
}

impl PartialOrd for Directory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0, other.0) {
            (a, b) if a == b => Some(Ordering::Equal),
            ('$', _) => Some(Ordering::Less),
            (_, '$') => Some(Ordering::Greater),
            (a, b) => Some(a.cmp(&b)),
        }
    }
}

impl TryFrom<char> for Directory {
    type Error = Error;

    fn try_from(value: char) -> StdResult<Self, Self::Error> {
        if !is_file_name_char(value) {
            bail!("invalid directory {value}")
        }

        Ok(Self(value))
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{c}", c = self.0)
    }
}

impl<'de> Deserialize<'de> for Directory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = char::deserialize(deserializer)?;
        s.try_into().map_err(SerdeError::custom)
    }
}
