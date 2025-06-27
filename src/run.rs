use crate::args::{Args, Command};
use crate::detokenize_command::do_detokenize;
use crate::extract_command::do_extract;
use crate::make_command::do_make;
use crate::show_command::do_show;
use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    match Args::parse().command {
        Command::Detokenize { path, overwrite } => do_detokenize(&path, overwrite)?,
        Command::Extract {
            ssd_path,
            overwrite,
        } => do_extract(&ssd_path, overwrite)?,
        Command::Make {
            ssd_path,
            overwrite,
        } => do_make(&ssd_path, overwrite)?,
        Command::Show { ssd_path } => do_show(&ssd_path)?,
    }
    Ok(())
}
