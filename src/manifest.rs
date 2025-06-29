use crate::manifest_file::ManifestFile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    #[serde(rename = "version")]
    pub version: u32,

    #[serde(rename = "files")]
    pub files: Vec<ManifestFile>,
}
