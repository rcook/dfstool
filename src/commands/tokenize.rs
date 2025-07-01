use crate::bbc_basic::tokenize_source;
use crate::util::open_for_write;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn run_tokenize(input_path: &Path, output_path: &Path, overwrite: bool) -> Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut input_bytes = Vec::new();
    input_file.read_to_end(&mut input_bytes)?;
    tokenize_source(open_for_write(output_path, overwrite)?, &input_bytes)
}
