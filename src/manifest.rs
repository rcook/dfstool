use crate::disc_size::DiscSize;
use crate::disc_title::DiscTitle;
use crate::manifest_file::ManifestFile;
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

    #[serde(
        rename = "discSize",
        alias = "disc_size",
        default = "Manifest::default_disc_size"
    )]
    pub disc_size: DiscSize,

    #[serde(rename = "files")]
    pub files: Vec<ManifestFile>,
}

impl Manifest {
    pub fn default_disc_size() -> DiscSize {
        800.try_into().unwrap()
    }
}
