use crate::dfs::{Address, Directory, FileName, FileSpec};
use crate::metadata::{File, FileType};
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileDescriptor {
    pub file_name: FileName,
    pub directory: Directory,
    pub locked: bool,
    pub load_address: Address,
    pub execution_address: Address,
}

impl FileDescriptor {
    pub const fn new(
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

    pub fn content_path(&self) -> PathBuf {
        let mut s = String::with_capacity(11);
        if !self.directory.is_root() {
            s.push(self.directory.into());
            s.push('.');
        }
        s.push_str(self.file_name.as_str());
        PathBuf::from(s)
    }

    pub fn to_manifest_file(&self, file_type: FileType) -> File {
        File {
            file_name: self.file_name.clone(),
            directory: self.directory,
            locked: self.locked,
            load_address: self.load_address,
            execution_address: self.execution_address,
            content_path: self.content_path(),
            r#type: file_type,
        }
    }
}

impl FileSpec for FileDescriptor {
    fn directory(&self) -> &Directory {
        &self.directory
    }

    fn file_name(&self) -> &FileName {
        &self.file_name
    }
}
