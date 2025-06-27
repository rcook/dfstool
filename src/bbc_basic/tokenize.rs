use crate::bbc_basic::{END_MARKER, KEYWORDS_BY_NAME};
use anyhow::{Result, anyhow, bail};
use std::io::Write;

pub fn tokenize_source<W: Write>(mut writer: W, source: &str) -> Result<()> {
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        tokenize_line(&mut writer, line)?;
    }

    writer.write_all(&END_MARKER)?;

    Ok(())
}

fn parse_line_number_and_content(line: &str) -> Result<(u16, &str)> {
    let (line_number_str, content) = match line.find(|c: char| !c.is_ascii_digit()) {
        Some(index) => line.split_at(index),
        None => (line, &line[line.len()..]),
    };

    if line_number_str.is_empty() {
        bail!("no line number in {line}")
    }

    let line_number = line_number_str
        .parse::<u16>()
        .map_err(|_| anyhow!("invalid line number: {}", line_number_str))?;

    Ok((line_number, content))
}

fn tokenize_line<W: Write>(mut writer: W, line: &str) -> Result<()> {
    let (line_number, content) = parse_line_number_and_content(line)?;
    let tokens = tokenize_content(content)?;

    let line_len = tokens.len() as u8 + 4;
    writer.write_all(&[0x0d])?;
    writer.write_all(&[(line_number >> 8) as u8, (line_number & 0xff) as u8])?;
    writer.write_all(&[line_len])?;
    writer.write_all(&tokens)?;
    Ok(())
}

fn tokenize_content(content: &str) -> Result<Vec<u8>> {
    let mut chars = content.chars().peekable();

    let mut output = Vec::new();
    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                // String literal
                output.push(ch as u8);
                for next_ch in chars.by_ref() {
                    output.push(next_ch as u8);
                    if next_ch == '"' {
                        break;
                    }
                }
            }
            '0'..='9' => {
                // TBD: Properly tokenize line numbers!
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
                let mut word = String::new();
                word.push(ch);

                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '$' || next_ch == '(' {
                        word.push(chars.next().expect("already peeked"));
                    } else {
                        break;
                    }
                }

                if let Some(token) = find_keyword_token(&word) {
                    output.push(token);
                } else {
                    for c in word.chars() {
                        output.push(c as u8);
                    }
                }
            }
            _ => {
                output.push(ch as u8);
            }
        }
    }

    Ok(output)
}

fn find_keyword_token(word: &str) -> Option<u8> {
    KEYWORDS_BY_NAME.get(word).copied()
}
