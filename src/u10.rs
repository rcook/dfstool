use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use anyhow::Result;

macro_rules! u10 {
    ($name: ident) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name(u16);

        impl $name {
            #[allow(unused)]
            pub fn as_usize(&self) -> usize {
                self.0 as usize
            }

            #[allow(unused)]
            pub fn as_u64(&self) -> u64 {
                self.0 as u64
            }
        }

        impl std::convert::TryFrom<u16> for $name {
            type Error = anyhow::Error;

            fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
                if value > 0x3ff {
                    anyhow::bail!("cannot convert {value} to u10")
                }
                Ok(Self(value))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{value:03X}", value = self.0)
            }
        }
    };
}

u10!(DiscSize);
u10!(StartSector);

impl DiscSize {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        let lo_bits = bytes[SECTOR_SIZE + 7];
        let temp = bytes[SECTOR_SIZE + 6];
        assert_eq!(0, temp & 0b11001100);
        let hi_bits = ((temp & 0b00000011) as u16) << 8;
        let disc_size = hi_bits + lo_bits as u16;
        assert!((2..=1023).contains(&disc_size));
        disc_size.try_into()
    }

    pub fn write_to(&self, bytes: &mut [u8]) -> Result<()> {
        let hi = (self.0 >> 8) as u8;
        let lo = (self.0 & 0xff) as u8;
        bytes[SECTOR_SIZE + 7] = lo;
        bytes[SECTOR_SIZE + 6] |= hi;
        Ok(())
    }
}
