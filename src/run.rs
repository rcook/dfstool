use crate::args::{Args, Command};
use crate::commands::{
    ExtractOpts, run_detokenize, run_extract, run_make, run_manifest, run_new, run_show,
    run_tokenize,
};
use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    match Args::parse().command {
        Command::Detokenize {
            path,
            output_path,
            overwrite,
            lossless,
        } => run_detokenize(&path, output_path.as_ref(), overwrite, lossless)?,
        Command::Extract {
            path,
            output_dir,
            overwrite,
            no_detokenize,
            lossless,
            inf,
        } => run_extract(
            &path,
            &output_dir,
            &ExtractOpts {
                overwrite,
                detokenize: !no_detokenize,
                lossless,
                inf,
            },
        )?,
        Command::Make {
            path,
            side_1_path,
            output_path,
            overwrite,
        } => run_make(&path, side_1_path.as_ref(), &output_path, overwrite)?,
        Command::New {
            output_path,
            disc_size,
            overwrite,
        } => run_new(&output_path, disc_size, overwrite)?,
        Command::Manifest {
            dir,
            output_path,
            overwrite,
        } => run_manifest(&dir, output_path.as_ref(), overwrite)?,
        Command::Show { path } => run_show(&path)?,
        Command::Tokenize {
            path,
            output_path,
            overwrite,
        } => run_tokenize(&path, &output_path, overwrite)?,
    }
    Ok(())
}
