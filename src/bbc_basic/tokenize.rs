use crate::bbc_basic::{
    END_MARKER, KEYWORDS_BY_NAME, LINE_NUMBER_TOKEN, LINE_NUMBER_TOKENS, REM_TOKEN, TokenGenerator,
    encode_line_number,
};
use crate::line_ending::LineEnding;
use anyhow::{Result, anyhow, bail};
use std::io::Write;

pub fn tokenize_source<W: Write>(mut writer: W, bytes: &[u8]) -> Result<()> {
    for line in LineEnding::guess(bytes).lines(bytes) {
        tokenize_line(&mut writer, line)?;
    }
    writer.write_all(&END_MARKER)?;
    Ok(())
}

fn tokenize_line<W: Write>(mut writer: W, bytes: &[u8]) -> Result<()> {
    let (line_number, bytes) = parse_line_number(bytes)?;
    let tokens = tokenize_content(bytes)?;
    let line_len = u8::try_from(tokens.len() + 4)?;
    writer.write_all(&[0x0d])?;
    writer.write_all(&[(line_number >> 8) as u8, (line_number & 0xff) as u8])?;
    writer.write_all(&[line_len])?;
    writer.write_all(&tokens)?;
    Ok(())
}

fn parse_line_number(bytes: &[u8]) -> Result<(u16, &[u8])> {
    let mut i = 0;
    let len = bytes.len();

    // Skip whitespace
    while i < len && (bytes[i] as char).is_ascii_whitespace() {
        i += 1;
    }

    // Grab digits
    let mut j = i;
    if !(bytes[j] as char).is_ascii_digit() {
        bail!("line number missing from source line")
    }
    let mut line_number = u16::from(bytes[j] - b'0');
    j += 1;
    while j < len && (bytes[j] as char).is_ascii_digit() {
        line_number = line_number
            .checked_mul(10)
            .and_then(|value| value.checked_add(u16::from(bytes[j] - b'0')))
            .ok_or_else(|| anyhow!("invalid line number",))?;
        j += 1;
    }

    Ok((line_number, &bytes[j..]))
}

fn tokenize_content(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut generator = TokenGenerator::new(bytes);
    while let Some(byte) = generator.peek() {
        process_byte(&mut generator, byte)?;
    }
    Ok(generator.drain_output())
}

fn process_byte(generator: &mut TokenGenerator<'_>, byte: u8) -> Result<(), anyhow::Error> {
    use crate::bbc_basic::TokenGeneratorState::{Comment, LineNumber, Other};

    let ch = byte as char;
    match (generator.state(), ch) {
        (Other, '"') => {
            generator.push_next_assert();
            while let Some(byte) = generator.next() {
                generator.push(byte);
                let c = byte as char;
                if c == '"' {
                    break;
                }
            }
        }
        (Comment | LineNumber, '0'..='9') => {
            let line_number = read_line_number(generator)?;
            let (byte0, byte1, byte2) = encode_line_number(line_number);
            generator.push(LINE_NUMBER_TOKEN);
            generator.push(byte0);
            generator.push(byte1);
            generator.push(byte2);
            if matches!(generator.state(), LineNumber) {
                generator.set_state(Other);
            }
        }
        (Other, '0'..='9') => {
            generator.push_next_assert();
            while let Some(byte) = generator.peek() {
                let c = byte as char;
                if !c.is_ascii_digit() && c != '.' {
                    break;
                }
                generator.push_next_assert();
            }
        }
        (Other, 'A'..='Z' | 'a'..='z') => {
            struct TokenRun {
                index: usize,
                token: u8,
            }

            let s = {
                let mut s = String::new();
                generator.next_assert();
                s.push(ch);
                while let Some(byte) = generator.peek() {
                    let c = byte as char;
                    if !c.is_ascii_alphabetic() && c != '$' && c != '(' {
                        break;
                    }
                    s.push(generator.next_assert() as char);
                }
                s
            };

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
                }
            }

            for run in runs {
                match generator.state() {
                    Comment => {}
                    LineNumber => generator.set_state(Other),
                    Other => {
                        if run.token == REM_TOKEN {
                            generator.set_state(Comment);
                        } else if LINE_NUMBER_TOKENS.contains(&run.token) {
                            generator.set_state(LineNumber);
                        }
                    }
                }
                generator.push(run.token);
            }

            let remainder = &s[start..];
            if !remainder.is_empty() {
                for c in remainder.chars() {
                    generator.push(c as u8);
                }
            }
        }
        _ => generator.push_next_assert(),
    }
    Ok(())
}

fn read_line_number(generator: &mut TokenGenerator<'_>) -> Result<u16> {
    let mut line_number = u16::from(generator.next_assert() - b'0');
    while let Some(byte) = generator.peek() {
        let c = byte as char;
        if !c.is_ascii_digit() {
            break;
        }

        generator.next_assert();

        line_number = line_number
            .checked_mul(10)
            .and_then(|value| value.checked_add(u16::from(byte - b'0')))
            .ok_or_else(|| anyhow!("invalid line number",))?;
    }
    Ok(line_number)
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

    const PROG1_STR: &str = concat!(
        "   10MODE 7\n\r",
        "   20DIM code% 256\n\r",
        "   30FOR opt% = 0 TO 2 STEP 2\n\r",
        "   40P% = code%\n\r",
        "   50[OPT opt%\n\r",
        "   60 LDA #65\n\r",
        "   70 JSR &FFEE\n\r",
        "   80 RTS\n\r",
        "   90]\n\r",
        "  100NEXT\n\r",
        "  110CALL code%\n\r"
    );

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

    const PROG2_STR: &str = concat!(
        "   10MODE 7\n\r",
        "   20PRINT TAB(5) \"HELLO\"\n\r",
        "   30GOSUB 140\n\r",
        "   40GOTO 60\n\r",
        "   50PRINT \"SKIP THIS\"\n\r",
        "   60FOR a% = 0 TO 4\n\r",
        "   70IF a% = 0 THEN 80 ELSE 100\n\r",
        "   80PRINT \"a% is zero\"\n\r",
        "   90GOTO 110\n\r",
        "  100PRINT \"a% is nonzero\"\n\r",
        "  110PRINT \"ENDIF\"\n\r",
        "  120NEXT\n\r",
        "  130END\n\r",
        "  140PRINT \"A SUBROUTINE\"\n\r",
        "  150RETURN\n\r"
    );

    const PROG3: [u8; 85] = [
        0x0d, 0x00, 0x0a, 0x0f, 0xf1, 0x8a, 0x35, 0x29, 0x22, 0x48, 0x45, 0x4c, 0x4c, 0x4f, 0x22,
        0x0d, 0x00, 0x14, 0x08, 0xf2, 0x53, 0x55, 0x42, 0x0d, 0x00, 0x1e, 0x0a, 0xf1, 0xa4, 0x46,
        0x55, 0x4e, 0x43, 0x0d, 0x00, 0x28, 0x05, 0xe0, 0x0d, 0x00, 0x32, 0x09, 0xdd, 0xf2, 0x53,
        0x55, 0x42, 0x0d, 0x00, 0x3c, 0x0a, 0xf1, 0x22, 0x53, 0x55, 0x42, 0x22, 0x0d, 0x00, 0x46,
        0x05, 0xe1, 0x0d, 0x00, 0x50, 0x0a, 0xdd, 0xa4, 0x46, 0x55, 0x4e, 0x43, 0x0d, 0x00, 0x5a,
        0x0b, 0x3d, 0x22, 0x46, 0x55, 0x4e, 0x43, 0x22, 0x0d, 0xff,
    ];

    const PROG3_STR: &str = concat!(
        "   10PRINTTAB(5)\"HELLO\"\n\r",
        "   20PROCSUB\n\r",
        "   30PRINTFNFUNC\n\r",
        "   40END\n\r",
        "   50DEFPROCSUB\n\r",
        "   60PRINT\"SUB\"\n\r",
        "   70ENDPROC\n\r",
        "   80DEFFNFUNC\n\r",
        "   90=\"FUNC\"\n\r"
    );

    const PROG4: [u8; 272] = [
        0x0d, 0x00, 0x00, 0xd2, 0xf4, 0x22, 0x16, 0x07, 0x84, 0x9d, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x84, 0x9d, 0x83, 0x9d, 0x84, 0x8d, 0x20, 0x20, 0x20, 0x20, 0x20, 0x54,
        0x48, 0x45, 0x20, 0x53, 0x54, 0x41, 0x49, 0x52, 0x57, 0x41, 0x59, 0x20, 0x54, 0x4f, 0x20,
        0x48, 0x45, 0x4c, 0x4c, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x84, 0x9d, 0x20, 0x84, 0x9d,
        0x83, 0x9d, 0x84, 0x8d, 0x20, 0x20, 0x20, 0x20, 0x20, 0x54, 0x48, 0x45, 0x20, 0x53, 0x54,
        0x41, 0x49, 0x52, 0x57, 0x41, 0x59, 0x20, 0x54, 0x4f, 0x20, 0x48, 0x45, 0x4c, 0x4c, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x84, 0x9d, 0x20, 0x84, 0x9d, 0x83, 0x9d, 0x84, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x77, 0x77, 0x77, 0x2e, 0x73, 0x74, 0x61, 0x69, 0x72, 0x77, 0x61, 0x79,
        0x74, 0x6f, 0x68, 0x65, 0x6c, 0x6c, 0x2e, 0x63, 0x6f, 0x6d, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x84, 0x9d, 0x20, 0x84, 0x9d, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x0d, 0x00, 0x0a, 0x0a, 0x20, 0x2a, 0x45, 0x58, 0x45, 0x43, 0x0d, 0x00, 0x14, 0x08, 0x20,
        0xeb, 0x20, 0x37, 0x0d, 0x00, 0x1e, 0x0e, 0x20, 0x2a, 0x46, 0x58, 0x20, 0x32, 0x30, 0x30,
        0x2c, 0x32, 0x0d, 0x00, 0x28, 0x0c, 0x20, 0xd0, 0x3d, 0x26, 0x31, 0x39, 0x30, 0x30, 0x0d,
        0x00, 0x32, 0x10, 0x20, 0xd7, 0x20, 0x22, 0x42, 0x2e, 0x45, 0x4c, 0x49, 0x54, 0x45, 0x22,
        0x0d, 0xff,
    ];

    const PROG4_STR: &str = concat!(
        "    0REM\"\u{16}\u{07}\u{84}\u{9D}                                      \u{84}\u{9D}\u{83}\u{9D}\u{84}\u{8D}     THE STAIRWAY TO HELL      \u{84}\u{9D} \u{84}\u{9D}\u{83}\u{9D}\u{84}\u{8D}     THE STAIRWAY TO HELL      \u{84}\u{9D} \u{84}\u{9D}\u{83}\u{9D}\u{84}     www.stairwaytohell.com     \u{84}\u{9D} \u{84}\u{9D}                                        \n\r",
        "   10 *EXEC\n\r",
        "   20 MODE 7\n\r",
        "   30 *FX 200,2\n\r",
        "   40 PAGE=&1900\n\r",
        "   50 CHAIN \"B.ELITE\"\n\r"
    );

    #[rstest]
    #[case(PROG1_STR, &PROG1)]
    #[case(PROG2_STR, &PROG2)]
    #[case(PROG3_STR, &PROG3)]
    #[case(PROG4_STR, &PROG4)]
    fn detokenize(#[case] expected_source: &str, #[case] input_token_bytes: &[u8]) -> Result<()> {
        let expected_source_bytes = get_source_bytes(expected_source);
        let mut source_bytes = Vec::new();
        detokenize_source(Cursor::new(&mut source_bytes), input_token_bytes, true)?;
        assert!(!source_bytes.contains(&0xc2));
        assert_eq!(expected_source_bytes, source_bytes);
        Ok(())
    }

    #[rstest]
    #[case(&PROG1, PROG1_STR)]
    #[case(&PROG2, PROG2_STR)]
    #[case(&PROG3, PROG3_STR)]
    #[case(&PROG4, PROG4_STR)]
    fn tokenize(#[case] expected_token_bytes: &[u8], #[case] input_source: &str) -> Result<()> {
        let input_bytes = get_source_bytes(input_source);
        let mut token_bytes = Vec::new();
        tokenize_source(Cursor::new(&mut token_bytes), &input_bytes)?;
        assert_eq!(expected_token_bytes, token_bytes);
        Ok(())
    }

    #[rstest]
    #[case(&[0xe0], "END")]
    #[case(&[0xe1], "ENDPROC")]
    #[case(&[0xe1, 0xed], "ENDPROCNEXT")]
    #[case(&[0xe1, 0xed, 0xec], "ENDPROCNEXTMOVE")]
    #[case(&[0xe4, 0xe0], "GOSUBEND")]
    #[case(&[0xe4, 0xe1, 0xe5], "GOSUBENDPROCGOTO")]
    fn tokenize_content_basics(
        #[case] expected_token_bytes: &[u8],
        #[case] input_source: &str,
    ) -> Result<()> {
        let input_bytes = get_source_bytes(input_source);
        let token_bytes = tokenize_content(&input_bytes)?;
        assert_eq!(expected_token_bytes, token_bytes);
        Ok(())
    }

    fn get_source_bytes(source: &str) -> Vec<u8> {
        // Cast characters individually to u8 in order to throw away
        // the 0xc2 Unicode control character
        let bytes = source.chars().map(|c| c as u8).collect::<Vec<_>>();
        assert!(!bytes.contains(&0xc2));
        bytes
    }
}
