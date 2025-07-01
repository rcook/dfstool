use crate::dfs::DiscSize;
use clap::{Parser, Subcommand};
use path_absolutize::Absolutize;
use std::path::PathBuf;
use std::result::Result as StdResult;

#[derive(Debug, Parser)]
#[command(
    name = "dfstool",
    about = "Acorn DFS Disc Image Manager",
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
        #[arg(help = "Path to BBC BASIC file", required = true, value_parser = parse_absolute_path)]
        path: PathBuf,

        #[arg(help = "Path to output text file", long = "output", short = 'o', value_parser = parse_absolute_path)]
        output_path: Option<PathBuf>,

        #[arg(
            help = "Overwrite output file if it already exists",
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

    #[command(name = "extract", about = "Extract files and metadata from disc image")]
    Extract {
        #[arg(help = "Path to disc image file", required = true, value_parser = parse_absolute_path)]
        path: PathBuf,

        #[arg(help = "Path to output directory", required = true, value_parser = parse_absolute_path)]
        output_dir: PathBuf,

        #[arg(
            help = "Overwrite output files if they already exist",
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

        #[arg(
            help = "Generate .inf files instead of storing metadata in manifest",
            long = "inf",
            short = 'i',
            default_value_t = false
        )]
        inf: bool,
    },

    #[command(name = "make", about = "Make disc image from files and metadata")]
    Make {
        #[arg(help = "Path to manifest", required = true, value_parser = parse_absolute_path)]
        path: PathBuf,

        #[arg(help = "Path to output disc image file", required = true, value_parser = parse_absolute_path)]
        output_path: PathBuf,

        #[arg(
            help = "Overwrite output file if it already exists",
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
        #[arg(help = "Path to content directory", required = true, value_parser = parse_absolute_path)]
        dir: PathBuf,

        #[arg(help = "Path to output manifest file", long = "output", short = 'o', value_parser = parse_absolute_path)]
        output_path: Option<PathBuf>,

        #[arg(
            help = "Overwrite output file if it already exists",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,
    },

    #[command(name = "new", about = "Create a new, empty disc image file")]
    New {
        #[arg(help = "Path to output disc image file", value_parser = parse_absolute_path)]
        output_path: PathBuf,

        #[arg(help = "Size of new disc image in sectors", value_parser = parse_disc_size)]
        disc_size: Option<DiscSize>,

        #[arg(
            help = "Overwrite output file if it already exists",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,
    },

    #[command(name = "show", about = "Show catalogue")]
    Show {
        #[arg(help = "Path to disc image file", required = true, value_parser = parse_absolute_path)]
        path: PathBuf,
    },

    #[command(name = "tokenize", about = "Tokenize BBC BASIC program")]
    Tokenize {
        #[arg(help = "Path to input text file", required = true, value_parser = parse_absolute_path)]
        path: PathBuf,

        #[arg(help = "Path to BBC BASIC output file", required = true, value_parser = parse_absolute_path)]
        output_path: PathBuf,

        #[arg(
            help = "Overwrite output file if it already exists",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,
    },
}

fn parse_absolute_path(s: &str) -> StdResult<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}

fn parse_disc_size(s: &str) -> StdResult<DiscSize, String> {
    s.parse::<u16>()
        .map_err(|_| String::from("invalid disc size"))?
        .try_into()
        .map_err(|_| String::from("invalid disc size"))
}
