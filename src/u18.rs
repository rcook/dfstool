macro_rules! u18 {
    ($name: ident) => {
        #[derive(Debug, serde::Serialize)]
        pub struct $name(u32);

        impl $name {
            #[allow(unused)]
            pub fn as_usize(&self) -> usize {
                self.0 as usize
            }
        }

        impl std::convert::TryFrom<u32> for $name {
            type Error = anyhow::Error;

            fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
                if value > 0x3fffff {
                    anyhow::bail!("cannot convert {value} to u18")
                }
                Ok(Self(value))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{value:06X}", value = self.0)
            }
        }
    };
}

u18!(Address);
u18!(Length);
