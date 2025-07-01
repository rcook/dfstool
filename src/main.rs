mod args;
mod bbc_basic;
mod commands;
mod constants;
mod dfs;
mod image_reader;
mod line_ending;
mod metadata;
mod ops;
mod path_util;
mod run;
mod ssd_reader;
mod u10;
mod u18;
mod util;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
