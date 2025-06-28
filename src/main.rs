#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::pedantic)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::verbose_bit_mask)]
#![allow(missing_docs)]

mod address;
mod args;
mod bbc_basic;
mod boot_option;
mod catalogue;
mod catalogue_bytes;
mod catalogue_entry;
mod constants;
mod cycle_number;
mod detokenize_command;
mod directory;
mod disc_size;
mod disc_title;
mod extract_command;
mod file_count;
mod file_descriptor;
mod file_name;
mod file_offset;
mod file_type;
mod length;
mod line_parser;
mod make_command;
mod manifest;
mod manifest_file;
mod run;
mod show_command;
mod start_sector;
mod tokenize_command;
mod u10;
mod u18;
mod util;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
