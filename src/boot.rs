#[derive(Debug)]
#[repr(u8)]
pub enum Boot {
    None = 0,
    Load = 1,
    Run = 2,
    Exec = 3,
}

impl Boot {
    pub fn from_byte(byte: u8) -> Self {
        match (byte & 0b00110000) >> 4 {
            0 => Self::None,
            1 => Self::Load,
            2 => Self::Run,
            3 => Self::Exec,
            _ => unimplemented!(),
        }
    }
}
