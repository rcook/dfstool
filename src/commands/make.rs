use crate::constants::MANIFEST_VERSION;
use crate::metadata::Manifest;
use crate::ops::new_ssd;
use anyhow::{Result, anyhow, bail};
use std::fs::File;
use std::path::Path;

pub fn run_make(manifest_path: &Path, output_ssd_path: &Path, overwrite: bool) -> Result<()> {
    let manifest_dir = manifest_path.parent().ok_or_else(|| {
        anyhow!(
            "cannot get parent directory from {manifest_path}",
            manifest_path = manifest_path.display()
        )
    })?;

    let manifest_file = File::open(manifest_path)?;
    let manifest = serde_json::from_reader::<_, Manifest>(manifest_file)?;
    if let Some(version) = manifest.version
        && version != MANIFEST_VERSION
    {
        bail!("unsupported manifest version {version}");
    }

    new_ssd(output_ssd_path, overwrite, manifest_dir, manifest)
}
