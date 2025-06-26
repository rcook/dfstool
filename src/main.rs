mod args;
mod boot_option;
mod catalogue;
mod catalogue_bytes;
mod catalogue_entry;
mod constants;
mod cycle_number;
mod detokenize;
mod directory;
mod disc_title;
mod extract;
mod file_count;
mod file_descriptor;
mod file_name;
mod file_offset;
mod make;
mod run;
mod show;
mod u10;
mod u18;
mod util;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
