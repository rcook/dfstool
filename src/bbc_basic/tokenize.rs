use crate::bbc_basic::{
    END_MARKER, KEYWORDS_BY_NAME, LINE_NUMBER_TOKEN, LINE_NUMBER_TOKENS, encode_line_number,
};
use anyhow::{Result, anyhow, bail};
use std::io::Write;

pub fn tokenize_source<W: Write>(mut writer: W, source: &str) -> Result<()> {
    // BBC BASIC sources must be strictly ASCII!
    if !source.is_ascii() {
        bail!("invalid source {source}")
    }

    for line in source.lines() {
        let line = line.trim();
        if !line.is_empty() {
            tokenize_line(&mut writer, line)?;
        }
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
    macro_rules! next {
        ($bytes: ident, $iter: ident) => {
            if $iter < $bytes.len() {
                let index = $iter;
                $iter += 1;
                Some($bytes[index])
            } else {
                None
            }
        };
    }

    macro_rules! peek {
        ($bytes: ident, $iter: ident) => {
            if $iter < $bytes.len() {
                let index = $iter;
                Some($bytes[index])
            } else {
                None
            }
        };
    }

    // Source must be pure ASCII so we can treat it as a byte array
    // and eliminate copying of characters
    assert!(content.is_ascii());
    let bytes = content.as_bytes();

    let mut previous_token = None;
    let mut output = Vec::new();
    let mut iter = 0;
    while let Some(byte) = next!(bytes, iter) {
        let ch = byte as char;
        match ch {
            '"' => {
                output.push(byte);
                while let Some(byte) = next!(bytes, iter) {
                    output.push(byte);
                    let c = byte as char;
                    if c == '"' {
                        break;
                    }
                }
            }
            '0'..='9' => match previous_token {
                Some(token) if LINE_NUMBER_TOKENS.contains(&token) => {
                    let line_number = {
                        let mut acc = (byte - 48) as u16;
                        while let Some(byte) = peek!(bytes, iter) {
                            let c = byte as char;
                            if !c.is_ascii_digit() {
                                break;
                            }

                            next!(bytes, iter).unwrap();

                            acc = acc
                                .checked_mul(10)
                                .and_then(|value| value.checked_add((byte - 48) as u16))
                                .ok_or_else(|| anyhow!("invalid line number in {content}"))?;
                        }
                        acc
                    };

                    previous_token = None;

                    let (byte0, byte1, byte2) = encode_line_number(line_number);
                    output.push(LINE_NUMBER_TOKEN);
                    output.push(byte0);
                    output.push(byte1);
                    output.push(byte2);
                }
                _ => {
                    output.push(byte);
                    while let Some(byte) = peek!(bytes, iter) {
                        let c = byte as char;
                        if !c.is_ascii_digit() && c != '.' {
                            break;
                        }
                        output.push(next!(bytes, iter).unwrap());
                    }
                }
            },
            'A'..='Z' | 'a'..='z' => {
                let s = {
                    let mut s = String::new();
                    s.push(ch);
                    while let Some(byte) = peek!(bytes, iter) {
                        let c = byte as char;
                        if !c.is_ascii_alphabetic() && c != '$' && c != '(' {
                            break;
                        }
                        s.push(next!(bytes, iter).unwrap() as char);
                    }
                    s
                };

                struct TokenRun {
                    index: usize,
                    token: u8,
                }

                // Convert ENDPROC to single token instead of two etc.
                let mut runs: Vec<TokenRun> = Vec::new();
                let mut start = 0;
                for i in 0..=s.len() {
                    if !runs.is_empty() {
                        let index = runs.len() - 1;
                        let run = &runs[index];
                        let word = &s[run.index..i];
                        if let Some(token) = find_token(word) {
                            runs[index].token = token;
                            start = i;
                            continue;
                        }
                    }

                    let word = &s[start..i];
                    if let Some(token) = find_token(word) {
                        runs.push(TokenRun {
                            index: start,
                            token,
                        });
                        start = i;
                        continue;
                    }
                }

                for run in runs {
                    output.push(run.token);
                    previous_token = Some(run.token);
                }

                let remainder = &s[start..];
                if !remainder.is_empty() {
                    for c in remainder.chars() {
                        output.push(c as u8);
                    }
                    previous_token = None;
                }
            }
            _ => output.push(byte),
        }
    }

    Ok(output)
}

fn find_token(word: &str) -> Option<u8> {
    KEYWORDS_BY_NAME.get(word).copied()
}

#[cfg(test)]
mod tests {
    use crate::bbc_basic::tokenize::tokenize_content;
    use crate::bbc_basic::{detokenize_source, tokenize_source};
    use anyhow::Result;
    use rstest::rstest;
    use std::io::Cursor;

    const PROG1: [u8; 128] = [
        0x0d, 0x00, 0x0a, 0x07, 0xeb, 0x20, 0x37, 0x0d, 0x00, 0x14, 0x0f, 0xde, 0x20, 0x63, 0x6f,
        0x64, 0x65, 0x25, 0x20, 0x32, 0x35, 0x36, 0x0d, 0x00, 0x1e, 0x16, 0xe3, 0x20, 0x6f, 0x70,
        0x74, 0x25, 0x20, 0x3d, 0x20, 0x30, 0x20, 0xb8, 0x20, 0x32, 0x20, 0x88, 0x20, 0x32, 0x0d,
        0x00, 0x28, 0x0e, 0x50, 0x25, 0x20, 0x3d, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x25, 0x0d, 0x00,
        0x32, 0x0d, 0x5b, 0x4f, 0x50, 0x54, 0x20, 0x6f, 0x70, 0x74, 0x25, 0x0d, 0x00, 0x3c, 0x0c,
        0x20, 0x4c, 0x44, 0x41, 0x20, 0x23, 0x36, 0x35, 0x0d, 0x00, 0x46, 0x0e, 0x20, 0x4a, 0x53,
        0x52, 0x20, 0x26, 0x46, 0x46, 0x45, 0x45, 0x0d, 0x00, 0x50, 0x08, 0x20, 0x52, 0x54, 0x53,
        0x0d, 0x00, 0x5a, 0x05, 0x5d, 0x0d, 0x00, 0x64, 0x05, 0xed, 0x0d, 0x00, 0x6e, 0x0b, 0xd6,
        0x20, 0x63, 0x6f, 0x64, 0x65, 0x25, 0x0d, 0xff,
    ];

    const PROG1_STR: &str = r#"   10MODE 7
   20DIM code% 256
   30FOR opt% = 0 TO 2 STEP 2
   40P% = code%
   50[OPT opt%
   60 LDA #65
   70 JSR &FFEE
   80 RTS
   90]
  100NEXT
  110CALL code%
"#;

    const PROG2: [u8; 202] = [
        0x0d, 0x00, 0x0a, 0x07, 0xeb, 0x20, 0x37, 0x0d, 0x00, 0x14, 0x11, 0xf1, 0x20, 0x8a, 0x35,
        0x29, 0x20, 0x22, 0x48, 0x45, 0x4c, 0x4c, 0x4f, 0x22, 0x0d, 0x00, 0x1e, 0x0a, 0xe4, 0x20,
        0x8d, 0x74, 0x4c, 0x40, 0x0d, 0x00, 0x28, 0x0a, 0xe5, 0x20, 0x8d, 0x54, 0x7c, 0x40, 0x0d,
        0x00, 0x32, 0x11, 0xf1, 0x20, 0x22, 0x53, 0x4b, 0x49, 0x50, 0x20, 0x54, 0x48, 0x49, 0x53,
        0x22, 0x0d, 0x00, 0x3c, 0x10, 0xe3, 0x20, 0x61, 0x25, 0x20, 0x3d, 0x20, 0x30, 0x20, 0xb8,
        0x20, 0x34, 0x0d, 0x00, 0x46, 0x1a, 0xe7, 0x20, 0x61, 0x25, 0x20, 0x3d, 0x20, 0x30, 0x20,
        0x8c, 0x20, 0x8d, 0x44, 0x50, 0x40, 0x20, 0x8b, 0x20, 0x8d, 0x44, 0x64, 0x40, 0x0d, 0x00,
        0x50, 0x12, 0xf1, 0x20, 0x22, 0x61, 0x25, 0x20, 0x69, 0x73, 0x20, 0x7a, 0x65, 0x72, 0x6f,
        0x22, 0x0d, 0x00, 0x5a, 0x0a, 0xe5, 0x20, 0x8d, 0x44, 0x6e, 0x40, 0x0d, 0x00, 0x64, 0x15,
        0xf1, 0x20, 0x22, 0x61, 0x25, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x6e, 0x7a, 0x65, 0x72,
        0x6f, 0x22, 0x0d, 0x00, 0x6e, 0x0d, 0xf1, 0x20, 0x22, 0x45, 0x4e, 0x44, 0x49, 0x46, 0x22,
        0x0d, 0x00, 0x78, 0x05, 0xed, 0x0d, 0x00, 0x82, 0x05, 0xe0, 0x0d, 0x00, 0x8c, 0x14, 0xf1,
        0x20, 0x22, 0x41, 0x20, 0x53, 0x55, 0x42, 0x52, 0x4f, 0x55, 0x54, 0x49, 0x4e, 0x45, 0x22,
        0x0d, 0x00, 0x96, 0x05, 0xf8, 0x0d, 0xff,
    ];

    const PROG2_STR: &str = r#"   10MODE 7
   20PRINT TAB(5) "HELLO"
   30GOSUB 140
   40GOTO 60
   50PRINT "SKIP THIS"
   60FOR a% = 0 TO 4
   70IF a% = 0 THEN 80 ELSE 100
   80PRINT "a% is zero"
   90GOTO 110
  100PRINT "a% is nonzero"
  110PRINT "ENDIF"
  120NEXT
  130END
  140PRINT "A SUBROUTINE"
  150RETURN
"#;

    const PROG3: [u8; 85] = [
        0x0d, 0x00, 0x0a, 0x0f, 0xf1, 0x8a, 0x35, 0x29, 0x22, 0x48, 0x45, 0x4c, 0x4c, 0x4f, 0x22,
        0x0d, 0x00, 0x14, 0x08, 0xf2, 0x53, 0x55, 0x42, 0x0d, 0x00, 0x1e, 0x0a, 0xf1, 0xa4, 0x46,
        0x55, 0x4e, 0x43, 0x0d, 0x00, 0x28, 0x05, 0xe0, 0x0d, 0x00, 0x32, 0x09, 0xdd, 0xf2, 0x53,
        0x55, 0x42, 0x0d, 0x00, 0x3c, 0x0a, 0xf1, 0x22, 0x53, 0x55, 0x42, 0x22, 0x0d, 0x00, 0x46,
        0x05, 0xe1, 0x0d, 0x00, 0x50, 0x0a, 0xdd, 0xa4, 0x46, 0x55, 0x4e, 0x43, 0x0d, 0x00, 0x5a,
        0x0b, 0x3d, 0x22, 0x46, 0x55, 0x4e, 0x43, 0x22, 0x0d, 0xff,
    ];

    const PROG3_STR: &str = r#"   10PRINTTAB(5)"HELLO"
   20PROCSUB
   30PRINTFNFUNC
   40END
   50DEFPROCSUB
   60PRINT"SUB"
   70ENDPROC
   80DEFFNFUNC
   90="FUNC"
"#;

    #[rstest]
    #[case(PROG1_STR, &PROG1)]
    #[case(PROG2_STR, &PROG2)]
    #[case(PROG3_STR, &PROG3)]
    fn detokenize(#[case] expected_output: &str, #[case] input: &[u8]) -> Result<()> {
        let mut bytes = Vec::new();
        detokenize_source(Cursor::new(&mut bytes), input)?;
        let s = String::from_utf8(bytes)?;
        assert_eq!(expected_output, s);
        Ok(())
    }

    #[rstest]
    #[case(&PROG1, PROG1_STR)]
    #[case(&PROG2, PROG2_STR)]
    #[case(&PROG3, PROG3_STR)]
    fn tokenize(#[case] expected_output: &[u8], #[case] input: &str) -> Result<()> {
        let mut bytes = Vec::new();
        tokenize_source(Cursor::new(&mut bytes), input)?;
        assert_eq!(expected_output, bytes);
        Ok(())
    }

    #[rstest]
    #[case(&[0xe0], "END")]
    #[case(&[0xe1], "ENDPROC")]
    #[case(&[0xe1, 0xed], "ENDPROCNEXT")]
    #[case(&[0xe1, 0xed, 0xec], "ENDPROCNEXTMOVE")]
    #[case(&[0xe4, 0xe0], "GOSUBEND")]
    #[case(&[0xe4, 0xe1, 0xe5], "GOSUBENDPROCGOTO")]
    fn tokenize_content_basics(#[case] expected_output: &[u8], #[case] input: &str) -> Result<()> {
        let bytes = tokenize_content(input)?;
        assert_eq!(expected_output, bytes);
        Ok(())
    }
}
