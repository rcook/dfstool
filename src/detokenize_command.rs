use crate::detokenize::detokenize_source;
use crate::util::open_for_write;
use anyhow::{Result, bail};
use std::fs::{File, remove_file};
use std::io::{Read, stdout};
use std::path::{Path, PathBuf};

pub fn do_detokenize(
    input_path: &Path,
    output_path: &Option<PathBuf>,
    overwrite: bool,
) -> Result<()> {
    let mut f = File::open(input_path)?;
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)?;

    let result = match output_path {
        Some(output_path) => detokenize_source(open_for_write(output_path, overwrite)?, &bytes),
        None => detokenize_source(stdout(), &bytes),
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
