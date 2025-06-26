use anyhow::bail;
use std::convert::TryFrom;
use std::result::Result as StdResult;

#[derive(Debug)]
#[repr(u8)]
pub enum BootOption {
    None = 0,
    Load = 1,
    Run = 2,
    Exec = 3,
}

impl TryFrom<u8> for BootOption {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        Ok(match (value & 0b00110000) >> 4 {
            0 => Self::None,
            1 => Self::Load,
            2 => Self::Run,
            3 => Self::Exec,
            _ => bail!("invalid boot option {value}"),
        })
    }
}
