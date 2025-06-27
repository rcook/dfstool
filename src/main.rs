mod address;
mod args;
mod boot_option;
mod catalogue;
mod catalogue_bytes;
mod catalogue_entry;
mod constants;
mod cycle_number;
mod detokenize;
mod detokenize_command;
mod directory;
mod disc_size;
mod disc_title;
mod extract_command;
mod file_count;
mod file_descriptor;
mod file_name;
mod file_offset;
mod length;
mod make_command;
mod run;
mod show_command;
mod start_sector;
mod u10;
mod u18;
mod util;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
