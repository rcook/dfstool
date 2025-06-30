use anyhow::{Result, anyhow};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub fn has_extension(path: &Path, ext: &str) -> bool {
    path.extension().and_then(OsStr::to_str) == Some(ext)
}

pub fn strip_extension(path: &Path) -> Result<PathBuf> {
    let dir = path.parent().ok_or_else(|| {
        anyhow!(
            "could not get directory from path {path}",
            path = path.display()
        )
    })?;

    let stem = path
        .file_stem()
        .ok_or_else(|| anyhow!("could not get stem from path {path}", path = path.display()))?;

    Ok(dir.join(stem))
}

pub fn add_extension(path: &Path, ext: &str) -> Result<PathBuf> {
    let dir = path.parent().ok_or_else(|| {
        anyhow!(
            "could not get directory from path {path}",
            path = path.display()
        )
    })?;

    let mut file_name = path
        .file_name()
        .ok_or_else(|| anyhow!("could not get stem from path {path}", path = path.display()))?
        .to_os_string();
    file_name.push(".");
    file_name.push(ext);

    Ok(dir.join(file_name))
}
