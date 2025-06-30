use crate::address::Address;
use crate::directory::Directory;
use crate::file_descriptor::FileDescriptor;
use crate::file_name::FileName;
use crate::file_spec::FileSpec;
use crate::file_type::FileType;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestFile {
    #[serde(rename = "fileName", alias = "file_name")]
    pub file_name: FileName,

    #[serde(rename = "directory")]
    pub directory: Directory,

    #[serde(rename = "locked")]
    pub locked: bool,

    #[serde(rename = "loadAddress", alias = "load_address")]
    pub load_address: Address,

    #[serde(rename = "executionAddress", alias = "execution_address")]
    pub execution_address: Address,

    #[serde(rename = "contentPath", alias = "content_path")]
    pub content_path: PathBuf,

    #[serde(rename = "type")]
    pub r#type: FileType,
}

impl ManifestFile {
    pub fn to_file_descriptor(&self) -> FileDescriptor {
        FileDescriptor {
            file_name: self.file_name.clone(),
            directory: self.directory,
            locked: self.locked,
            load_address: self.load_address,
            execution_address: self.execution_address,
        }
    }
}

impl FileSpec for ManifestFile {
    fn directory(&self) -> &Directory {
        &self.directory
    }

    fn file_name(&self) -> &FileName {
        &self.file_name
    }
}
