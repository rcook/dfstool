use crate::bbc_basic::tokenize_source;
use crate::util::open_for_write;
use anyhow::{Result, bail};
use std::fs::{File, remove_file};
use std::io::Read;
use std::path::Path;

pub fn do_tokenize(input_path: &Path, output_path: &Path, overwrite: bool) -> Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut input_bytes = Vec::new();
    input_file.read_to_end(&mut input_bytes)?;

    match tokenize_source(open_for_write(output_path, overwrite)?, &input_bytes) {
        Ok(()) => Ok(()),
        Err(e) => {
            remove_file(output_path)?;
            bail!(e)
        }
    }
}
