pub const CR: u8 = 0x0d; // 13

pub const LF: u8 = 0x0a; // 10

#[derive(Clone, Copy, Debug)]
pub enum LineEnding {
    Cr,   // native Acorn line ending (*BUILD)
    LfCr, // native Acorn line ending (*SPOOL)
    CrLf, // native Windows line ending
    Lf,   // native Posix line ending: default if best guess impossible
}

impl LineEnding {
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

    pub const fn lines(self, bytes: &[u8]) -> Lines<'_> {
        Lines::new(self, bytes)
    }
}

pub struct Lines<'a> {
    line_ending: LineEnding,
    bytes: &'a [u8],
    ptr: usize,
    len: usize,
    line_start: usize,
    previous_byte: Option<u8>,
}

impl<'a> Lines<'a> {
    const fn new(line_ending: LineEnding, bytes: &'a [u8]) -> Self {
        Self {
            line_ending,
            bytes,
            ptr: 0,
            len: bytes.len(),
            line_start: 0,
            previous_byte: None,
        }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        use crate::line_ending::LineEnding::{Cr, CrLf, Lf, LfCr};

        while self.ptr < self.len {
            assert!(self.ptr <= self.len);
            assert!(self.line_start <= self.ptr);
            assert!(self.ptr == 0 && self.previous_byte.is_none() || self.previous_byte.is_some());

            let byte = self.bytes[self.ptr];
            let line = match (&self.line_ending, self.previous_byte, byte) {
                (Cr, _, CR) | (Lf, _, LF) => {
                    let line = &self.bytes[self.line_start..self.ptr];
                    self.line_start = self.ptr + 1;
                    Some(line)
                }
                (CrLf, Some(CR), LF) | (LfCr, Some(LF), CR) => {
                    let line = &self.bytes[self.line_start..self.ptr - 1];
                    self.line_start = self.ptr + 1;
                    Some(line)
                }
                (CrLf, _, LF) | (LfCr, _, CR) => {
                    panic!("invalid line ending at position {ptr}", ptr = self.ptr)
                }
                _ => None,
            };

            self.previous_byte = Some(byte);
            self.ptr += 1;

            if line.is_some() {
                return line;
            }
        }

        None
    }
}
