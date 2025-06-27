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

// https://xania.org/200711/bbc-basic-v-format
// https://xania.org/200711/BBCBasicToText.py
// https://www.bbcbasic.net/wiki/doku.php?id=detokeniser
const KEYWORDS: [&str; 129] = [
    "OTHERWISE",
    "AND",
    "DIV",
    "EOR",
    "MOD",
    "OR",
    "ERROR",
    "LINE",
    "OFF",
    "STEP",
    "SPC",
    "TAB(",
    "ELSE",
    "THEN",
    "<line>",
    "OPENIN",
    "PTR",
    "PAGE",
    "TIME",
    "LOMEM",
    "HIMEM",
    "ABS",
    "ACS",
    "ADVAL",
    "ASC",
    "ASN",
    "ATN",
    "BGET",
    "COS",
    "COUNT",
    "DEG",
    "ERL",
    "ERR",
    "EVAL",
    "EXP",
    "EXT",
    "FALSE",
    "FN",
    "GET",
    "INKEY",
    "INSTR(",
    "INT",
    "LEN",
    "LN",
    "LOG",
    "NOT",
    "OPENUP",
    "OPENOUT",
    "PI",
    "POINT(",
    "POS",
    "RAD",
    "RND",
    "SGN",
    "SIN",
    "SQR",
    "TAN",
    "TO",
    "TRUE",
    "USR",
    "VAL",
    "VPOS",
    "CHR$",
    "GET$",
    "INKEY$",
    "LEFT$(",
    "MID$(",
    "RIGHT$(",
    "STR$",
    "STRING$(",
    "EOF",
    "<ESCFN>",
    "<ESCCOM>",
    "<ESCSTMT>",
    "WHEN",
    "OF",
    "ENDCASE",
    "ELSE",
    "ENDIF",
    "ENDWHILE",
    "PTR",
    "PAGE",
    "TIME",
    "LOMEM",
    "HIMEM",
    "SOUND",
    "BPUT",
    "CALL",
    "CHAIN",
    "CLEAR",
    "CLOSE",
    "CLG",
    "CLS",
    "DATA",
    "DEF",
    "DIM",
    "DRAW",
    "END",
    "ENDPROC",
    "ENVELOPE",
    "FOR",
    "GOSUB",
    "GOTO",
    "GCOL",
    "IF",
    "INPUT",
    "LET",
    "LOCAL",
    "MODE",
    "MOVE",
    "NEXT",
    "ON",
    "VDU",
    "PLOT",
    "PRINT",
    "PROC",
    "READ",
    "REM",
    "REPEAT",
    "REPORT",
    "RESTORE",
    "RETURN",
    "RUN",
    "STOP",
    "COLOUR",
    "TRACE",
    "UNTIL",
    "WIDTH",
    "OSCLI",
];

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
                let keyword = KEYWORDS[*value as usize - 0x7f];
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

pub fn tokenize_source<W: Write>(mut writer: W, source: &str) -> Result<()> {
    let lines: Vec<&str> = source.lines().collect();
    
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // Parse line number and content
        let (line_number, content) = parse_line_number_and_content(line)?;
        
        // Tokenize the line content
        let mut tokenized_line = Vec::new();
        tokenize_line_content(&mut tokenized_line, content)?;
        
        // Write line header: 0x0d + line number (2 bytes) + line length + content
        let line_len = tokenized_line.len() as u8 + 4; // +4 for line number (2) + length (1) + 0x0d (1)
        writer.write_all(&[0x0d])?;
        writer.write_all(&[(line_number >> 8) as u8, (line_number & 0xff) as u8])?;
        writer.write_all(&[line_len])?;
        writer.write_all(&tokenized_line)?;
    }
    
    // Write end marker
    writer.write_all(&[0x0d, 0xff])?;
    
    Ok(())
}

fn parse_line_number_and_content(line: &str) -> Result<(u16, &str)> {
    let mut parts = line.splitn(2, char::is_whitespace);
    let line_num_str = parts.next().ok_or_else(|| anyhow::anyhow!("empty line"))?;
    let content = parts.next().unwrap_or("").trim();
    
    let line_number = line_num_str.parse::<u16>()
        .map_err(|_| anyhow::anyhow!("invalid line number: {}", line_num_str))?;
    
    Ok((line_number, content))
}

fn tokenize_line_content(output: &mut Vec<u8>, content: &str) -> Result<()> {
    let mut chars = content.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\t' => {
                // Skip whitespace
                continue;
            }
            '"' => {
                // String literal
                output.push(ch as u8);
                while let Some(next_ch) = chars.next() {
                    output.push(next_ch as u8);
                    if next_ch == '"' {
                        break;
                    }
                }
            }
            '0'..='9' => {
                // Number literal
                output.push(ch as u8);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' {
                        output.push(chars.next().unwrap() as u8);
                    } else {
                        break;
                    }
                }
            }
            'A'..='Z' | 'a'..='z' => {
                // Keyword or identifier
                let mut word = String::new();
                word.push(ch.to_ascii_uppercase());
                
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '$' || next_ch == '(' {
                        word.push(chars.next().unwrap().to_ascii_uppercase());
                    } else {
                        break;
                    }
                }
                
                // Check if it's a keyword
                if let Some(token) = find_keyword_token(&word) {
                    output.push(token);
                } else {
                    // It's an identifier, write as-is
                    for c in word.chars() {
                        output.push(c as u8);
                    }
                }
            }
            _ => {
                // Other characters
                output.push(ch as u8);
            }
        }
    }
    
    Ok(())
}

fn find_keyword_token(keyword: &str) -> Option<u8> {
    // Create a mapping from keywords to their token values
    // The token values are the index + 0x7f (as seen in detokenize)
    for (index, &kw) in KEYWORDS.iter().enumerate() {
        if keyword == kw {
            return Some((index + 0x7f) as u8);
        }
    }
    None
}
