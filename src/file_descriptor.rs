use crate::{
    directory::Directory,
    file_name::FileName,
    u18::{Address, Length},
};

#[derive(Debug)]
pub struct FileDescriptor {
    pub file_name: FileName,
    pub directory: Directory,
    pub locked: bool,
    pub load_address: Address,
    pub execution_address: Address,
    pub length: Length,
}

impl FileDescriptor {
    pub fn new(
        file_name: FileName,
        directory: Directory,
        locked: bool,
        load_address: Address,
        execution_address: Address,
        length: Length,
    ) -> Self {
        Self {
            file_name,
            directory,
            locked,
            load_address,
            execution_address,
            length,
        }
    }
}
