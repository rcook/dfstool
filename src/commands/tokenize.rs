use crate::bbc_basic::tokenize_source;
use crate::util::open_for_write;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn run_tokenize(path: &Path, output_path: &Path, overwrite: bool) -> Result<()> {
    let mut f = File::open(path)?;
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)?;
    tokenize_source(open_for_write(output_path, overwrite)?, &bytes)
}
