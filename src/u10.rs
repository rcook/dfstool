#[macro_export]
macro_rules! u10 {
    ($name: ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
        pub struct $name(u16);

        impl $name {
            pub const ZERO: Self = Self(0);

            // https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
            pub const fn to_u16(self) -> u16 {
                self.0
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0.checked_add(rhs.0).expect("must not overflow"))
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 = self.0.checked_add(rhs.0).expect("must not overflow");
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

        impl std::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::UpperHex::fmt(&self.0, f)
            }
        }

        impl std::convert::From<$name> for u16 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::convert::From<$name> for u32 {
            fn from(value: $name) -> Self {
                u32::from(value.0)
            }
        }

        impl std::convert::From<$name> for u64 {
            fn from(value: $name) -> Self {
                u64::from(value.0)
            }
        }

        impl std::convert::From<$name> for usize {
            fn from(value: $name) -> Self {
                usize::from(value.0)
            }
        }
    };
}
