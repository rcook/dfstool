use crate::{disc_title::DiscTitle, manifest_file::ManifestFile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,

    #[serde(
        rename = "discTitle",
        alias = "disc_title",
        skip_serializing_if = "Option::is_none"
    )]
    pub disc_title: Option<DiscTitle>,

    #[serde(rename = "files")]
    pub files: Vec<ManifestFile>,
}
