#[macro_export]
macro_rules! u18 {
    ($name: ident) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name(u32);

        impl $name {
            // https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
            pub const fn to_u32(self) -> u32 {
                self.0
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

        impl std::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::UpperHex::fmt(&self.0, f)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Debug, serde::Deserialize)]
                #[serde(untagged)]
                enum StringOrU32 {
                    String(String),
                    U32(u32),
                }

                let temp = match StringOrU32::deserialize(deserializer)? {
                    StringOrU32::String(s) => match s.strip_prefix('&') {
                        Some(suffix) => {
                            u32::from_str_radix(suffix, 16).map_err(serde::de::Error::custom)?
                        }
                        None => s.parse().map_err(serde::de::Error::custom)?,
                    },
                    StringOrU32::U32(value) => value,
                };
                let value = temp.try_into().map_err(serde::de::Error::custom)?;
                Ok(value)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&format!("&{value:06X}", value = self.0))
            }
        }

        impl std::convert::From<$name> for u32 {
            fn from(value: $name) -> Self {
                value.to_u32()
            }
        }
    };
}
