use crate::boot_option::BootOption;
use crate::cycle_number::CycleNumber;
use crate::disc_size::DiscSize;
use crate::manifest::Manifest;
use crate::ops::new_ssd;
use anyhow::{Result, anyhow};
use std::path::Path;

pub fn do_new(ssd_path: &Path, disc_size: Option<DiscSize>, overwrite: bool) -> Result<()> {
    let dir = ssd_path.parent().ok_or_else(|| {
        anyhow!(
            "could not get directory from path {path}",
            path = ssd_path.display()
        )
    })?;
    new_ssd(
        ssd_path,
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
