use crate::address::Address;
use std::sync::LazyLock;

pub static BBC_BASIC_2_EXECUTION_ADDRESS: LazyLock<Address> =
    LazyLock::new(|| 0x38023.try_into().expect("Must be valid"));
