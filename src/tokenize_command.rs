use crate::bbc_basic::tokenize_source;
use crate::util::open_for_write;
use anyhow::{Result, bail};
use std::fs::{File, remove_file};
use std::io::{Read, stdout};
use std::path::{Path, PathBuf};

pub fn do_tokenize(
    input_path: &Path,
    output_path: &Option<PathBuf>,
    overwrite: bool,
) -> Result<()> {
    let mut f = File::open(input_path)?;
    let mut source = String::new();
    f.read_to_string(&mut source)?;

    let result = match output_path {
        Some(output_path) => tokenize_source(open_for_write(output_path, overwrite)?, &source),
        None => tokenize_source(stdout(), &source),
    };

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            if let Some(output_path) = output_path {
                remove_file(output_path)?;
            }
            bail!(e)
        }
    }
} 