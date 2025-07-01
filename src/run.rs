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
            input_path,
            output_path,
            overwrite,
            lossless,
        } => run_detokenize(&input_path, output_path.as_ref(), overwrite, lossless)?,
        Command::Extract {
            input_path,
            output_dir,
            overwrite,
            no_detokenize,
            lossless,
            inf,
        } => run_extract(
            &input_path,
            &output_dir,
            &ExtractOpts {
                overwrite,
                detokenize: !no_detokenize,
                lossless,
                inf,
            },
        )?,
        Command::Make {
            manifest_path,
            output_path,
            overwrite,
        } => run_make(&manifest_path, &output_path, overwrite)?,
        Command::New {
            ssd_path,
            disc_size,
            overwrite,
        } => run_new(&ssd_path, disc_size, overwrite)?,
        Command::Manifest {
            dir,
            output_path,
            overwrite,
        } => run_manifest(&dir, output_path.as_ref(), overwrite)?,
        Command::Show { ssd_path } => run_show(&ssd_path)?,
        Command::Tokenize {
            input_path,
            output_path,
            overwrite,
        } => run_tokenize(&input_path, &output_path, overwrite)?,
    }
    Ok(())
}
