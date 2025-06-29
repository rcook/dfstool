use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use anyhow::{Result, bail};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct CycleNumber(u8);

impl CycleNumber {
    pub fn new(value: u8) -> Result<Self> {
        if !Self::is_in_range(value) {
            bail!("invalid cycle number {value}")
        }

        Ok(Self(value))
    }

    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        Self::new(Self::from_bcd(bytes[SECTOR_SIZE + 4])?)
    }

    pub fn write_to_catalogue(&self, bytes: &mut [u8]) -> Result<()> {
        bytes[SECTOR_SIZE + 4] = Self::to_bcd(self.0)?;
        Ok(())
    }

    const fn is_in_range(value: u8) -> bool {
        value <= 99
    }

    fn from_bcd(value: u8) -> Result<u8> {
        let hi = value >> 4;
        let lo = value & 0b0000_1111;
        if hi > 9 || lo > 9 {
            bail!("invalid BCD byte {value}")
        }

        Ok(hi * 10 + lo)
    }

    fn to_bcd(value: u8) -> Result<u8> {
        if !Self::is_in_range(value) {
            bail!("cannot convert {value} to BCD")
        }

        let (q, r) = (value / 10, value % 10);
        Ok((q << 4) + r)
    }
}

impl Display for CycleNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{value}", value = self.0)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rstest::rstest;

    use crate::cycle_number::CycleNumber;

    #[rstest]
    #[case(99, 0x99)]
    #[case(10, 0x10)]
    #[case(0, 0x00)]
    fn bcd_basics(#[case] expected_result: u8, #[case] input: u8) -> Result<()> {
        assert_eq!(expected_result, CycleNumber::from_bcd(input)?);
        assert_eq!(input, CycleNumber::to_bcd(expected_result)?);
        Ok(())
    }

    #[test]
    fn bcd_errors() {
        assert!(CycleNumber::from_bcd(0x0a).is_err());
        assert!(CycleNumber::to_bcd(101).is_err());
    }
}
