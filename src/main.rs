mod args;
mod bbc_basic;
mod commands;
mod constants;
mod dfs;
mod file_type;
mod inf;
mod line_ending;
mod manifest;
mod manifest_file;
mod ops;
mod path_util;
mod run;
mod u10;
mod u18;
mod util;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
