macro_rules! u10 {
    ($name: ident) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name(u16);

        impl $name {
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
