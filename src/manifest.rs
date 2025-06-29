use crate::manifest_file::ManifestFile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,

    #[serde(rename = "files")]
    pub files: Vec<ManifestFile>,
}
