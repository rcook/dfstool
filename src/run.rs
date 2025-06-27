use crate::args::{Args, Command};
use crate::detokenize_command::do_detokenize;
use crate::extract_command::do_extract;
use crate::make_command::do_make;
use crate::show_command::do_show;
use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    match Args::parse().command {
        Command::Detokenize {
            input_path,
            output_path,
            overwrite,
        } => do_detokenize(&input_path, &output_path, overwrite)?,
        Command::Extract {
            input_path,
            output_dir,
            overwrite,
        } => do_extract(&input_path, &output_dir, overwrite)?,
        Command::Make {
            input_dir,
            output_path,
            overwrite,
        } => do_make(&input_dir, &output_path, overwrite)?,
        Command::Show { ssd_path } => do_show(&ssd_path)?,
    }
    Ok(())
}
