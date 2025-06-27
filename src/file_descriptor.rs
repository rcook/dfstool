use crate::address::Address;
use crate::directory::Directory;
use crate::file_name::FileName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileDescriptor {
    pub file_name: FileName,
    pub directory: Directory,
    pub locked: bool,
    pub load_address: Address,
    pub execution_address: Address,
}

impl FileDescriptor {
    pub fn new(
        file_name: FileName,
        directory: Directory,
        locked: bool,
        load_address: Address,
        execution_address: Address,
    ) -> Self {
        Self {
            file_name,
            directory,
            locked,
            load_address,
            execution_address,
        }
    }
}
