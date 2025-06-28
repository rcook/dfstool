use crate::bbc_basic::{END_MARKER, TOKEN_MASK};
use anyhow::{Result, bail};
use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

pub fn is_token(value: u8) -> bool {
    (value & TOKEN_MASK) != 0
}

pub fn is_bbc_basic_file(content_path: &Path) -> Result<bool> {
    let mut f = File::open(content_path)?;
    match f.seek(SeekFrom::End(-2)) {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::InvalidInput => return Ok(false),
        Err(e) => bail!(e),
    }

    // https://www.bbcbasic.net/wiki/doku.php?id=format
    let mut bytes = [0; 2];
    f.read_exact(&mut bytes)?;
    if bytes != END_MARKER {
        return Ok(false);
    }

    Ok(true)
}
