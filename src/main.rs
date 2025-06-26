mod args;
mod bcd_value;
mod boot_option;
mod catalogue;
mod catalogue_bytes;
mod catalogue_entry;
mod constants;
mod directory;
mod disc_title;
mod extract;
mod file_descriptor;
mod file_name;
mod file_offset;
mod run;
mod show;
mod u10;
mod u18;
mod util;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
