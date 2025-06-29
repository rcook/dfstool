use crate::bbc_basic::is_bbc_basic_file;
use crate::constants::MANIFEST_VERSION;
use crate::directory::Directory;
use crate::disc_side::DISC_SIDE_0;
use crate::file_name::FileName;
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
    // If output path is not specified, then infer from the directory name
    let output_path = if let Some(p) = output_path {
        p
    } else {
        let file_name = dir
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| anyhow!("cannot get directory name"))?;
        &dir.join(format!("{file_name}.json"))
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

    let manifest_file = open_for_write(output_path, overwrite)?;
    serde_json::to_writer_pretty(
        manifest_file,
        &Manifest {
            version: Some(MANIFEST_VERSION),
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

    let file_name: FileName = if let Ok(f) = file_name_str.parse() {
        f
    } else {
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
        disc_side: *DISC_SIDE_0,
        locked: false,
        load_address: 0.try_into()?,
        execution_address: 0.try_into()?,
        content_path,
        r#type: file_type,
    }))
}
