mod address;
mod args;
mod bbc_basic;
mod boot_option;
mod catalogue;
mod catalogue_bytes;
mod catalogue_entry;
mod commands;
mod constants;
mod cycle_number;
mod dfs_path;
mod directory;
mod disc_size;
mod disc_title;
mod file_count;
mod file_descriptor;
mod file_name;
mod file_offset;
mod file_spec;
mod file_type;
mod inf;
mod length;
mod line_ending;
mod manifest;
mod manifest_file;
mod ops;
mod path_util;
mod run;
mod start_sector;
mod u10;
mod u18;
mod util;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
