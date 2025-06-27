use clap::{Parser, Subcommand};
use path_absolutize::Absolutize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(name = "detokenize", about = "Detokenize BBC BASIC program")]
    Detokenize {
        #[arg(required = true, value_parser = parse_absolute_path)]
        input_path: PathBuf,

        #[arg(long = "output", short = 'o', value_parser = parse_absolute_path)]
        output_path: Option<PathBuf>,

        #[arg(long = "overwrite", short = 'f', default_value_t = false)]
        overwrite: bool,
    },

    #[command(name = "tokenize", about = "Tokenize BBC BASIC program")]
    Tokenize {
        #[arg(required = true, value_parser = parse_absolute_path)]
        input_path: PathBuf,

        #[arg(long = "output", short = 'o', value_parser = parse_absolute_path)]
        output_path: Option<PathBuf>,

        #[arg(long = "overwrite", short = 'f', default_value_t = false)]
        overwrite: bool,
    },

    #[command(name = "extract", about = "Extract files and metadata from .ssd file")]
    Extract {
        #[arg(required = true, value_parser = parse_absolute_path)]
        input_path: PathBuf,

        #[arg(required = true, value_parser = parse_absolute_path)]
        output_dir: PathBuf,

        #[arg(long = "overwrite", short = 'f', default_value_t = false)]
        overwrite: bool,

        #[arg(short = 'n', default_value_t = false)]
        no_detokenize: bool,
    },

    #[command(name = "make", about = "Make .ssd file from files and metadata")]
    Make {
        #[arg(required = true, value_parser = parse_absolute_path)]
        input_dir: PathBuf,

        #[arg(required = true, value_parser = parse_absolute_path)]
        output_path: PathBuf,

        #[arg(long = "overwrite", short = 'f', default_value_t = false)]
        overwrite: bool,
    },

    #[command(name = "show", about = "Show catalogue")]
    Show {
        #[arg(required = true, value_parser = parse_absolute_path)]
        ssd_path: PathBuf,
    },
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}
