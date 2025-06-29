#[macro_export]
macro_rules! u10 {
    ($name: ident) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name(u16);

        impl $name {
            // https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
            pub const fn to_u16(self) -> u16 {
                self.0
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

        impl std::convert::From<$name> for u16 {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}
