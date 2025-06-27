use crate::address::Address;
use std::sync::LazyLock;

pub const START_SECTOR: usize = 2;

pub const SECTOR_SIZE: usize = 256;

pub const SSD_CONTENT_FILE_EXT: &str = "ssdfile";

pub const SSD_METADATA_FILE_EXT: &str = "ssdfile.json";

pub static BBC_BASIC_2_EXECUTION_ADDRESS: LazyLock<Address> =
    LazyLock::new(|| 0x38023.try_into().expect("Must be valid"));
