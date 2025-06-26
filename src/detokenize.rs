use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::path::Path;

macro_rules! next {
    ($bytes: expr, $index: expr) => {{
        let Some(value) = $bytes.get($index) else {
            anyhow::bail!("end of file")
        };
        $index += 1;
        value
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

pub fn do_detokenize(path: &Path, overwrite: bool) -> Result<()> {
    let dir = path.parent().ok_or_else(|| anyhow!("cannot get parent"))?;
    let file_name = path
        .file_name()
        .ok_or_else(|| anyhow!("cannot get file name"))?;

    let bas_path = dir.join(format!("{f}.bas", f = file_name.display()));
    let bas_file = open_for_write(&bas_path, overwrite)?;

    let mut f = File::open(path)?;
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)?;

    match detokenize_file(bas_file, &bytes) {
        Ok(()) => Ok(()),
        Err(e) => {
            remove_file(&bas_path)?;
            bail!(e)
        }
    }
}

fn detokenize_file<W: Write>(mut writer: W, bytes: &[u8]) -> Result<()> {
    let mut index = 0;
    while index < bytes.len() {
        let b0 = next!(bytes, index);
        if *b0 != 13 {
            bail!("syntax error")
        }

        let b0 = next!(bytes, index);
        if *b0 == 0xff {
            break;
        }

        let b1 = next!(bytes, index);
        let line_number = ((*b0 as u16) << 8) + *b1 as u16;
        let line_len = next!(bytes, index);
        let last = index + *line_len as usize - 4;
        detokenize_line(&mut writer, line_number, &bytes[index..last])?;
        index = last;
    }

    Ok(())
}

fn detokenize_line<W: Write>(mut writer: W, line_number: u16, bytes: &[u8]) -> Result<()> {
    write!(writer, "{line_number:>5}")?;
    for b in bytes {
        match b {
            0x8d => todo!(),
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
