use clap::{Parser, Subcommand};
use path_absolutize::Absolutize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "dfstool",
    about = "Acorn DFS .ssd File Manager",
    after_help = "Documentation: https://github.com/rcook/dfstool/blob/main/README.md"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(name = "detokenize", about = "Detokenize BBC BASIC program")]
    Detokenize {
        #[arg(help = "Input file", required = true, value_parser = parse_absolute_path)]
        input_path: PathBuf,

        #[arg(help = "Output file", long = "output", short = 'o', value_parser = parse_absolute_path)]
        output_path: Option<PathBuf>,

        #[arg(
            help = "Overwrite output file if it exists",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,

        #[arg(
            help = "Output in non-printable lossless format preserving all control characters etc.",
            long = "lossless",
            short = 'l',
            default_value_t = false
        )]
        lossless: bool,
    },

    #[command(name = "extract", about = "Extract files and metadata from .ssd file")]
    Extract {
        #[arg(help = "Input file", required = true, value_parser = parse_absolute_path)]
        input_path: PathBuf,

        #[arg(help = "Output directory", required = true, value_parser = parse_absolute_path)]
        output_dir: PathBuf,

        #[arg(
            help = "Overwrite output files if they exist",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,

        #[arg(
            help = "Don't detokenize BASIC programs",
            short = 'n',
            default_value_t = false
        )]
        no_detokenize: bool,

        #[arg(
            help = "Output BBC BASIC in non-printable lossless format preserving all control characters etc.",
            long = "lossless",
            short = 'l',
            default_value_t = false
        )]
        lossless: bool,
    },

    #[command(name = "make", about = "Make .ssd file from files and metadata")]
    Make {
        #[arg(help = "Manifest path", required = true, value_parser = parse_absolute_path)]
        manifest_path: PathBuf,

        #[arg(help = "Output path", required = true, value_parser = parse_absolute_path)]
        output_path: PathBuf,

        #[arg(
            help = "Overwrite output file if it exists",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,
    },

    #[command(
        name = "manifest",
        about = "Generate a manifest file for the content in a given directory"
    )]
    Manifest {
        #[arg(help = "Directory", required = true, value_parser = parse_absolute_path)]
        dir: PathBuf,

        #[arg(help = "Output file", long = "output", short = 'o', value_parser = parse_absolute_path)]
        output_path: Option<PathBuf>,

        #[arg(
            help = "Overwrite output file if it exists",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,
    },

    #[command(name = "show", about = "Show catalogue")]
    Show {
        #[arg(help = "Path to .ssd file", required = true, value_parser = parse_absolute_path)]
        ssd_path: PathBuf,
    },

    #[command(name = "tokenize", about = "Tokenize BBC BASIC program")]
    Tokenize {
        #[arg(help = "Input path", required = true, value_parser = parse_absolute_path)]
        input_path: PathBuf,

        #[arg(help = "Output path", required = true, value_parser = parse_absolute_path)]
        output_path: PathBuf,

        #[arg(
            help = "Overwrite output file if it exists",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,
    },
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}
