use crate::dfs::{BootOption, CycleNumber, DiscSize};
use crate::metadata::Manifest;
use crate::ops::new_ssd;
use anyhow::{Result, anyhow};
use std::path::Path;

pub fn run_new(output_ssd_path: &Path, disc_size: Option<DiscSize>, overwrite: bool) -> Result<()> {
    let dir = output_ssd_path.parent().ok_or_else(|| {
        anyhow!(
            "could not get directory from path {path}",
            path = output_ssd_path.display()
        )
    })?;
    new_ssd(
        output_ssd_path,
        overwrite,
        dir,
        Manifest {
            version: None,
            disc_title: None,
            disc_size: disc_size.unwrap_or_default(),
            boot_option: BootOption::None,
            cycle_number: CycleNumber::default(),
            inf_files: Vec::new(),
            files: Vec::new(),
        },
    )
}
