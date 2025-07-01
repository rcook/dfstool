use anyhow::{Error, bail};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Side0,
    Side1,
}

impl TryFrom<u8> for Side {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Side0),
            1 => Ok(Self::Side1),
            _ => bail!("value {value} is not a valid side"),
        }
    }
}

impl TryFrom<usize> for Side {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u8::try_from(value)?.try_into()
    }
}

impl From<Side> for u8 {
    fn from(value: Side) -> Self {
        match value {
            Side::Side0 => 0,
            Side::Side1 => 1,
        }
    }
}

impl From<Side> for usize {
    fn from(value: Side) -> Self {
        usize::from(u8::from(value))
    }
}
