use crate::constants::MANIFEST_VERSION;
use crate::metadata::Manifest;
use crate::ops::new_image_file;
use anyhow::{Result, anyhow, bail};
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn run_make(
    path: &Path,
    side_1_path: Option<&PathBuf>,
    output_path: &Path,
    overwrite: bool,
) -> Result<()> {
    if side_1_path.is_some() {
        todo!(".dsd not implemented yet")
    }

    let manifest_dir = path.parent().ok_or_else(|| {
        anyhow!(
            "cannot get parent directory from {path}",
            path = path.display()
        )
    })?;

    let f = File::open(path)?;
    let manifest = serde_json::from_reader::<_, Manifest>(f)?;
    if let Some(version) = manifest.version
        && version != MANIFEST_VERSION
    {
        bail!("unsupported manifest version {version}");
    }

    new_image_file(output_path, overwrite, manifest_dir, manifest)
}
