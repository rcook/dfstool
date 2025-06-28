use crate::bbc_basic::{
    KEYWORDS_BY_TOKEN, LINE_NUMBER_TOKEN, REM_TOKEN, decode_line_number, is_ascii_printable,
    is_token,
};
use crate::line_ending::{CR, LF};
use anyhow::{Result, bail};
use std::io::Write;

pub fn detokenize_source<W: Write>(mut writer: W, bytes: &[u8], lossless: bool) -> Result<()> {
    macro_rules! next {
        ($bytes: expr, $index: expr) => {{
            let Some(value) = $bytes.get($index) else {
                anyhow::bail!("end of file")
            };
            $index += 1;
            *value
        }};
    }

    let mut index = 0;
    while index < bytes.len() {
        let b0 = next!(bytes, index);
        if b0 != CR {
            bail!("syntax error: file is not valid tokenized BBC BASIC")
        }

        let b0 = next!(bytes, index);
        if b0 == 0xff {
            break;
        }

        let b1 = next!(bytes, index);
        let line_number = ((b0 as u16) << 8) + b1 as u16;
        let line_len = next!(bytes, index);
        let last = index + line_len as usize - 4;
        detokenize_line(&mut writer, line_number, &bytes[index..last], lossless)?;
        index = last;
    }

    Ok(())
}

fn detokenize_line<W: Write>(
    mut writer: W,
    line_number: u16,
    bytes: &[u8],
    lossless: bool,
) -> Result<()> {
    macro_rules! w {
        ($writer: expr, $byte: expr) => {
            $writer.write_all(&[$byte])?
        };
    }

    macro_rules! next {
        ($iter: expr) => {{
            let Some(value) = $iter.next() else {
                anyhow::bail!("end of file")
            };
            *value
        }};
    }

    write!(writer, "{line_number:>5}")?;
    let mut iter = bytes.iter();
    while let Some(b) = iter.next() {
        match *b {
            LINE_NUMBER_TOKEN => {
                let b0 = next!(iter);
                let b1 = next!(iter);
                let b2 = next!(iter);
                let line_number = decode_line_number(b0, b1, b2);
                write!(writer, "{line_number}")?;
            }
            token if is_token(token) => {
                let Some(keyword) = KEYWORDS_BY_TOKEN.get(&token) else {
                    bail!("unknown token 0x{token:02x}")
                };
                write!(writer, "{keyword}")?;

                if token == REM_TOKEN {
                    for &value in iter {
                        if lossless || is_ascii_printable(value) {
                            w!(writer, value);
                        }
                    }
                    break;
                }
            }
            value => {
                if lossless || is_ascii_printable(value) {
                    w!(writer, value);
                }
            }
        }
    }

    if lossless {
        writer.write_all(&[LF, CR])?;
    } else {
        w!(writer, LF);
    }
    Ok(())
}
