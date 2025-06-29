use crate::disc_title::DiscTitle;
use crate::manifest_file::ManifestFile;
use crate::{boot_option::BootOption, disc_size::DiscSize};
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

    #[serde(rename = "discSize", alias = "disc_size", default)]
    pub disc_size: DiscSize,

    #[serde(rename = "bootOption", alias = "boot_option", default)]
    pub boot_option: BootOption,

    #[serde(rename = "files")]
    pub files: Vec<ManifestFile>,
}
