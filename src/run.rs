use crate::args::{Args, Command};
use crate::commands::{
    ExtractOpts, do_detokenize, do_extract, do_make, do_manifest, do_new, do_show, do_tokenize,
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
        } => do_detokenize(&input_path, output_path.as_ref(), overwrite, lossless)?,
        Command::Extract {
            input_path,
            output_dir,
            overwrite,
            no_detokenize,
            lossless,
            inf,
        } => do_extract(
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
        } => do_make(&manifest_path, &output_path, overwrite)?,
        Command::New {
            ssd_path,
            disc_size,
            overwrite,
        } => do_new(&ssd_path, disc_size, overwrite)?,
        Command::Manifest {
            dir,
            output_path,
            overwrite,
        } => do_manifest(&dir, output_path.as_ref(), overwrite)?,
        Command::Show { ssd_path } => do_show(&ssd_path)?,
        Command::Tokenize {
            input_path,
            output_path,
            overwrite,
        } => do_tokenize(&input_path, &output_path, overwrite)?,
    }
    Ok(())
}
