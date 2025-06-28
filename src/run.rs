use crate::args::{Args, Command};
use crate::detokenize_command::do_detokenize;
use crate::extract_command::do_extract;
use crate::make_command::do_make;
use crate::show_command::do_show;
use crate::tokenize_command::do_tokenize;
use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    match Args::parse().command {
        Command::Detokenize {
            input_path,
            output_path,
            overwrite,
            lossless,
        } => do_detokenize(&input_path, output_path.as_ref(), overwrite, lossless)?,
        Command::Extract {
            input_path,
            output_dir,
            overwrite,
            no_detokenize,
            lossless,
        } => do_extract(
            &input_path,
            &output_dir,
            overwrite,
            !no_detokenize,
            lossless,
        )?,
        Command::Make {
            manifest_path,
            output_path,
            overwrite,
        } => do_make(&manifest_path, &output_path, overwrite)?,
        Command::Show { ssd_path } => do_show(&ssd_path)?,
        Command::Tokenize {
            input_path,
            output_path,
            overwrite,
        } => do_tokenize(&input_path, &output_path, overwrite)?,
    }
    Ok(())
}
