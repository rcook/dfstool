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
            bbc_path,
            output_text_path,
            overwrite,
            lossless,
        } => run_detokenize(&bbc_path, output_text_path.as_ref(), overwrite, lossless)?,
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
            manifest_path,
            output_ssd_path,
            overwrite,
        } => run_make(&manifest_path, &output_ssd_path, overwrite)?,
        Command::New {
            output_ssd_path,
            disc_size,
            overwrite,
        } => run_new(&output_ssd_path, disc_size, overwrite)?,
        Command::Manifest {
            content_dir,
            output_manifest_path,
            overwrite,
        } => run_manifest(&content_dir, output_manifest_path.as_ref(), overwrite)?,
        Command::Show { ssd_path } => run_show(&ssd_path)?,
        Command::Tokenize {
            text_path,
            output_bbc_path,
            overwrite,
        } => run_tokenize(&text_path, &output_bbc_path, overwrite)?,
    }
    Ok(())
}
