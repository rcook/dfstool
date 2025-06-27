use crate::bbc_basic::BBC_BASIC_2_EXECUTION_ADDRESS;
use crate::file_descriptor::FileDescriptor;
use anyhow::{Result, bail};
use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

pub fn is_bbc_basic_file(content_path: &Path, descriptor: &FileDescriptor) -> Result<bool> {
    if descriptor.execution_address != *BBC_BASIC_2_EXECUTION_ADDRESS {
        return Ok(false);
    }

    let mut f = File::open(content_path)?;
    match f.seek(SeekFrom::End(-2)) {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::InvalidInput => return Ok(false),
        Err(e) => bail!(e),
    }

    // https://www.bbcbasic.net/wiki/doku.php?id=format
    let mut bytes = [0; 2];
    f.read_exact(&mut bytes)?;
    if bytes[0] != 0x0d || bytes[1] != 0xff {
        return Ok(false);
    }

    Ok(true)
}
