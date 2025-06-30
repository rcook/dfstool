use crate::bbc_basic::{END_MARKER, TOKEN_MASK};
use crate::line_ending::CR;
use anyhow::{Result, bail};
use std::fs::{File, metadata};
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

pub const fn is_token(value: u8) -> bool {
    (value & TOKEN_MASK) != 0
}

pub fn is_bbc_basic_file(content_path: &Path) -> Result<bool> {
    let m = metadata(content_path)?;
    if m.len() < 3 {
        return Ok(false);
    }

    let mut f = File::open(content_path)?;

    let mut bytes = [0; 1];
    f.read_exact(&mut bytes)?;
    if bytes[0] != CR {
        return Ok(false);
    }

    match f.seek(SeekFrom::End(-2)) {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::InvalidInput => return Ok(false),
        Err(e) => bail!(e),
    }

    // https://www.bbcbasic.net/wiki/doku.php?id=format
    let mut bytes = [0; 2];
    f.read_exact(&mut bytes)?;

    Ok(bytes == END_MARKER)
}
