use crate::detokenize::detokenize_source;
use crate::util::open_for_write;
use anyhow::{Result, bail};
use std::fs::{File, remove_file};
use std::io::Read;
use std::path::Path;

pub fn do_detokenize(input_path: &Path, output_path: &Path, overwrite: bool) -> Result<()> {
    let output_file = open_for_write(&output_path, overwrite)?;
    let mut f = File::open(input_path)?;
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)?;
    match detokenize_source(output_file, &bytes) {
        Ok(()) => Ok(()),
        Err(e) => {
            remove_file(&output_path)?;
            bail!(e)
        }
    }
}
