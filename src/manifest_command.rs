use crate::bbc_basic::is_bbc_basic_file;
use crate::boot_option::BootOption;
use crate::constants::MANIFEST_VERSION;
use crate::directory::Directory;
use crate::disc_size::DiscSize;
use crate::file_type::{FileType, KnownFileType};
use crate::manifest::Manifest;
use crate::manifest_file::ManifestFile;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use pathdiff::diff_paths;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub fn do_manifest(dir: &Path, output_path: Option<&PathBuf>, overwrite: bool) -> Result<()> {
    let dir_name = dir
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| anyhow!("cannot get directory name"))?;

    // If output path is not specified, then infer from the directory name
    let output_path = if let Some(p) = output_path {
        p
    } else {
        &dir.join(format!("{dir_name}.json"))
    };

    let manifest_dir = output_path
        .parent()
        .ok_or_else(|| anyhow!("cannot get parent directory"))?;

    let d = match read_dir(dir) {
        Ok(d) => d,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            bail!("directory {dir} not found", dir = dir.display())
        }
        Err(e) => bail!(e),
    };

    let mut files = Vec::new();
    for entry in d {
        let entry = entry?;

        let file_type = entry.file_type()?;
        if file_type.is_file()
            && let Some(file) = make_file(manifest_dir, &entry.path())?
        {
            files.push(file);
        }
    }

    let disc_title = match dir_name.parse() {
        Ok(t) => t,
        Err(_) => "Untitled".parse().unwrap(),
    };

    let manifest_file = open_for_write(output_path, overwrite)?;
    serde_json::to_writer_pretty(
        manifest_file,
        &Manifest {
            version: Some(MANIFEST_VERSION),
            disc_title: Some(disc_title),
            disc_size: DiscSize::default(),
            boot_option: BootOption::default(),
            files,
        },
    )?;

    Ok(())
}

fn make_file(manifest_dir: &Path, path: &Path) -> Result<Option<ManifestFile>> {
    let content_path =
        diff_paths(path, manifest_dir).ok_or_else(|| anyhow!("cannot build content path"))?;

    let file_name = path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| anyhow!("cannot get file name"))?;

    let (directory, file_name_str) = match file_name.split_once('.') {
        Some((prefix, suffix)) => {
            if prefix.len() == 1 {
                let c = prefix
                    .chars()
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap()
                    .to_owned();
                (c.try_into().unwrap_or(Directory::ROOT), suffix)
            } else {
                (Directory::ROOT, file_name)
            }
        }
        _ => (Directory::ROOT, file_name),
    };

    let Ok(file_name) = file_name_str.parse() else {
        eprintln!(
            "WARNING: Skipping file {path} since a valid DFS file name cannot be inferred",
            path = path.display()
        );
        return Ok(None);
    };

    let file_type = if is_bbc_basic_file(path)? {
        FileType::Known(KnownFileType::BbcBasic)
    } else {
        FileType::Known(KnownFileType::Other)
    };

    Ok(Some(ManifestFile {
        file_name,
        directory,
        locked: false,
        load_address: 0.try_into()?,
        execution_address: 0.try_into()?,
        content_path,
        r#type: file_type,
    }))
}
