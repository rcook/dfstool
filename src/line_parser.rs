use anyhow::{Result, bail};

pub const CR: u8 = 0x0d; // 13

pub const LF: u8 = 0x0a; // 10

#[derive(Debug)]
pub enum LineParser {
    Cr,   // native Acorn line ending (*BUILD)
    LfCr, // native Acorn line ending (*SPOOL)
    CrLf, // native Windows line ending
    Lf,   // native Posix line ending: default if best guess impossible
}

impl LineParser {
    pub fn guess(bytes: &[u8]) -> Self {
        let mut previous_byte = None;
        let mut ptr = 0;
        let len = bytes.len();
        while ptr < len {
            let byte = bytes[ptr];

            match (previous_byte, byte) {
                (Some(CR), LF) => return Self::CrLf,
                (Some(CR), _) => return Self::Cr,
                (Some(LF), CR) => return Self::LfCr,
                (Some(LF), _) => return Self::Lf,
                _ => {}
            }

            previous_byte = Some(byte);
            ptr += 1;
        }

        Self::Lf
    }

    // TBD: Return an iterator!
    pub fn lines<'a>(&self, bytes: &'a [u8]) -> Result<Vec<&'a [u8]>> {
        let mut slices = Vec::new();

        let mut previous_byte = None;
        let mut line_start = 0;
        let mut ptr = 0;
        let len = bytes.len();
        while ptr < len {
            let byte = bytes[ptr];

            match (self, previous_byte, byte) {
                (Self::Cr, _, CR) | (Self::Lf, _, LF) => {
                    let slice = &bytes[line_start..ptr];
                    slices.push(slice);
                    line_start = ptr + 1;
                }
                (Self::CrLf, Some(CR), LF) | (Self::LfCr, Some(LF), CR) => {
                    let slice = &bytes[line_start..ptr - 1];
                    slices.push(slice);
                    line_start = ptr + 1;
                }
                (Self::CrLf, _, LF) | (Self::LfCr, _, CR) => {
                    bail!("invalid line ending at position {ptr}")
                }
                _ => {}
            }

            previous_byte = Some(byte);
            ptr += 1;
        }

        Ok(slices)
    }
}
