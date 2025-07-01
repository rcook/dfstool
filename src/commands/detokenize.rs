use crate::bbc_basic::detokenize_source;
use crate::util::open_for_write;
use anyhow::Result;
use std::fs::File;
use std::io::{Read, stdout};
use std::path::{Path, PathBuf};

pub fn run_detokenize(
    input_path: &Path,
    output_path: Option<&PathBuf>,
    overwrite: bool,
    lossless: bool,
) -> Result<()> {
    let mut f = File::open(input_path)?;
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)?;
    match output_path {
        Some(output_path) => {
            detokenize_source(open_for_write(output_path, overwrite)?, &bytes, lossless)?;
        }
        None => detokenize_source(stdout(), &bytes, lossless)?,
    }
    Ok(())
}
