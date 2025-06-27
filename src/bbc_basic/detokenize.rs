use crate::bbc_basic::{KEYWORD_TOKEN_OFFSET, KEYWORDS};
use anyhow::{Result, bail};
use std::io::Write;

macro_rules! vec_next {
    ($bytes: expr, $index: expr) => {{
        let Some(value) = $bytes.get($index) else {
            anyhow::bail!("end of file")
        };
        $index += 1;
        *value
    }};
}

macro_rules! iter_next {
    ($iter: expr) => {{
        let Some(value) = $iter.next() else {
            anyhow::bail!("end of file")
        };
        *value
    }};
}

pub fn detokenize_source<W: Write>(mut writer: W, bytes: &[u8]) -> Result<()> {
    let mut index = 0;
    while index < bytes.len() {
        let b0 = vec_next!(bytes, index);
        if b0 != 13 {
            bail!("syntax error")
        }

        let b0 = vec_next!(bytes, index);
        if b0 == 0xff {
            break;
        }

        let b1 = vec_next!(bytes, index);
        let line_number = ((b0 as u16) << 8) + b1 as u16;
        let line_len = vec_next!(bytes, index);
        let last = index + line_len as usize - 4;
        detokenize_line(&mut writer, line_number, &bytes[index..last])?;
        index = last;
    }

    Ok(())
}

fn detokenize_line<W: Write>(mut writer: W, line_number: u16, bytes: &[u8]) -> Result<()> {
    write!(writer, "{line_number:>5}")?;
    let mut iter = bytes.iter();
    while let Some(b) = iter.next() {
        match b {
            0x8d => {
                // https://xania.org/200711/bbc-basic-line-number-format
                let b0 = iter_next!(iter);
                let b1 = iter_next!(iter);
                let b2 = iter_next!(iter);
                let line_number = decode_line_number(b0, b1, b2);
                write!(writer, "{line_number}")?;
            }
            value if (value & 0x80) != 0 => {
                let keyword = KEYWORDS[*value as usize - KEYWORD_TOKEN_OFFSET];
                write!(writer, "{keyword}")?
            }
            value => {
                if value.is_ascii_control() {
                    write!(writer, "[{value:02X}]")?
                } else {
                    write!(writer, "{c}", c = *value as char)?
                }
            }
        }
    }
    writeln!(writer)?;
    Ok(())
}

fn decode_line_number(b0: u8, b1: u8, b2: u8) -> u16 {
    let t0 = b0 ^ 0x54;
    let ll = (t0 & 0b00110000) >> 4;
    let hh = (t0 & 0b00001100) >> 2;

    let t1 = b1 & 0b00111111;
    let lo = t1 + (ll << 6);

    let t2 = b2 & 0b00111111;
    let hi = t2 + (hh << 6);

    ((hi as u16) << 8) + lo as u16
}
