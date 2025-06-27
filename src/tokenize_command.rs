use crate::bbc_basic::tokenize_source;
use crate::util::open_for_write;
use anyhow::{Result, bail};
use std::fs::{File, remove_file};
use std::io::Read;
use std::path::Path;

pub fn do_tokenize(input_path: &Path, output_path: &Path, overwrite: bool) -> Result<()> {
    let mut f = File::open(input_path)?;
    let mut source = String::new();
    f.read_to_string(&mut source)?;

    match tokenize_source(open_for_write(output_path, overwrite)?, &source) {
        Ok(()) => Ok(()),
        Err(e) => {
            remove_file(output_path)?;
            bail!(e)
        }
    }
}
