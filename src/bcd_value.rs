use anyhow::{Error, bail};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

#[derive(Debug)]
pub struct BcdValue(u8);

impl TryFrom<u8> for BcdValue {
    type Error = Error;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        let hi = value >> 4;
        let lo = value & 0b00001111;
        if hi > 9 || lo > 9 {
            bail!("invalid BCD byte {value}")
        }

        Ok(Self(hi * 10 + lo))
    }
}

impl Display for BcdValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{value}", value = self.0)
    }
}
