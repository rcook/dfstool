use crate::dfs::{BootOption, CycleNumber, DiscSize, DiscTitle};
use crate::metadata::File;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

    #[serde(rename = "cycleNumber", alias = "cycle_number", default)]
    pub cycle_number: CycleNumber,

    #[serde(
        rename = "infFiles",
        alias = "inf_files",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub inf_files: Vec<PathBuf>,

    #[serde(rename = "files", skip_serializing_if = "Vec::is_empty", default)]
    pub files: Vec<File>,
}
