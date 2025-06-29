use anyhow::{Error, bail};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::result::Result as StdResult;
use std::sync::LazyLock;

pub static DISC_SIDE_0: LazyLock<DiscSide> = LazyLock::new(|| 0.try_into().unwrap());
//pub static DISC_SIDE_1: LazyLock<DiscSide> = LazyLock::new(|| 1.try_into().unwrap());

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DiscSide(u8);

impl DiscSide {
    pub const fn to_u8(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for DiscSide {
    type Error = Error;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        if value > 1 {
            bail!("invalid disc side {value}")
        }

        Ok(Self(value))
    }
}

impl From<DiscSide> for u8 {
    fn from(value: DiscSide) -> Self {
        value.to_u8()
    }
}

impl<'de> Deserialize<'de> for DiscSide {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        value.try_into().map_err(SerdeError::custom)
    }
}
