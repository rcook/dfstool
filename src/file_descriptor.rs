use crate::address::Address;
use crate::directory::Directory;
use crate::file_name::FileName;
use crate::file_type::FileType;
use crate::manifest_file::ManifestFile;
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
        if self.directory.is_root() {
            PathBuf::from(self.file_name.to_string())
        } else {
            PathBuf::from(format!(
                "{directory}.{file_name}",
                directory = self.directory,
                file_name = self.file_name
            ))
        }
    }

    pub fn to_manifest_file(&self, file_type: FileType) -> ManifestFile {
        ManifestFile {
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
