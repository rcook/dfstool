use crate::directory::Directory;
use crate::disc_side::DISC_SIDE_0;
use crate::file_descriptor::FileDescriptor;
use crate::file_name::FileName;
use crate::file_type::FileType;
use crate::{address::Address, disc_side::DiscSide};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestFile {
    #[serde(rename = "file_name")]
    pub file_name: FileName,

    #[serde(rename = "directory")]
    pub directory: Directory,

    #[serde(rename = "disc_side", default = "ManifestFile::default_disc_side")]
    pub disc_side: DiscSide,

    #[serde(rename = "locked")]
    pub locked: bool,

    #[serde(rename = "load_address")]
    pub load_address: Address,

    #[serde(rename = "execution_address")]
    pub execution_address: Address,

    #[serde(rename = "content_path")]
    pub content_path: PathBuf,

    #[serde(rename = "type")]
    pub r#type: FileType,
}

impl ManifestFile {
    pub fn default_disc_side() -> DiscSide {
        *DISC_SIDE_0
    }

    pub fn to_file_descriptor(&self) -> FileDescriptor {
        FileDescriptor {
            file_name: self.file_name.clone(),
            directory: self.directory,
            disc_side: self.disc_side,
            locked: self.locked,
            load_address: self.load_address,
            execution_address: self.execution_address,
        }
    }
}

pub fn compare_by_file_spec(a: &ManifestFile, b: &ManifestFile) -> Ordering {
    match a.disc_side.partial_cmp(&b.disc_side) {
        Some(ordering) if ordering != Ordering::Equal => return ordering,
        _ => {}
    }
    match a.directory.partial_cmp(&b.directory) {
        Some(ordering) if ordering != Ordering::Equal => return ordering,
        _ => {}
    }
    match a.file_name.partial_cmp(&b.file_name) {
        Some(ordering) if ordering != Ordering::Equal => return ordering,
        _ => {}
    }
    Ordering::Equal
}
