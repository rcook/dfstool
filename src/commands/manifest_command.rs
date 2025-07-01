use crate::bbc_basic::is_bbc_basic_file;
use crate::constants::{INF_EXT, MANIFEST_VERSION};
use crate::dfs::{BootOption, CycleNumber, DfsPath, DiscSize, FileSpec};
use crate::metadata::{File, FileType, KnownFileType, Manifest};
use crate::path_util::{add_extension, has_extension, strip_extension};
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

    let mut inf_files = Vec::new();
    let mut files = Vec::new();
    for entry in d {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }

        let p = entry.path();

        if has_extension(&p, INF_EXT) {
            let content_path = strip_extension(&p)?;
            if content_path.is_file() {
                let file_name = content_path
                    .file_name()
                    .and_then(OsStr::to_str)
                    .ok_or_else(|| {
                        anyhow!(
                            "could not get file name from path {path}",
                            path = p.display()
                        )
                    })?;

                if let Ok(dfs_path) = file_name.parse::<DfsPath>() {
                    let rel_path = diff_paths(p, manifest_dir)
                        .ok_or_else(|| anyhow!("cannot build content path"))?;
                    inf_files.push((dfs_path, rel_path));
                } else {
                    eprintln!(
                        "WARNING: Skipping file {path} since a valid DFS file name cannot be inferred",
                        path = p.display()
                    );
                }
            } else {
                eprintln!(
                    "WARNING: Skipping {path} since corresponding content file does not exist",
                    path = p.display()
                );
            }
        } else {
            // Skip files with metadata stored in .inf files
            let inf_path = add_extension(&p, INF_EXT)?;
            if !inf_path.exists() {
                let file_name = p.file_name().and_then(OsStr::to_str).ok_or_else(|| {
                    anyhow!(
                        "could not get file name from path {path}",
                        path = p.display()
                    )
                })?;

                if let Ok(dfs_path) = file_name.parse() {
                    files.push(make_manifest_file(manifest_dir, &entry.path(), dfs_path)?);
                } else {
                    eprintln!(
                        "WARNING: Skipping file {path} since a valid DFS file name cannot be inferred",
                        path = p.display()
                    );
                }
            }
        }
    }

    let disc_title = match dir_name.parse() {
        Ok(t) => t,
        Err(_) => "Untitled".parse().unwrap(),
    };

    inf_files.sort_by(|a, b| FileSpec::compare(&a.0, &b.0));
    files.sort_by(FileSpec::compare);

    let manifest = open_for_write(output_path, overwrite)?;
    serde_json::to_writer_pretty(
        manifest,
        &Manifest {
            version: Some(MANIFEST_VERSION),
            disc_title: Some(disc_title),
            disc_size: DiscSize::default(),
            boot_option: BootOption::default(),
            cycle_number: CycleNumber::default(),
            inf_files: inf_files.into_iter().map(|x| x.1).collect(),
            files,
        },
    )?;

    Ok(())
}

fn make_manifest_file(manifest_dir: &Path, path: &Path, dfs_path: DfsPath) -> Result<File> {
    let content_path =
        diff_paths(path, manifest_dir).ok_or_else(|| anyhow!("cannot build content path"))?;

    let file_type = if is_bbc_basic_file(path)? {
        FileType::Known(KnownFileType::BbcBasic)
    } else {
        FileType::Known(KnownFileType::Other)
    };

    Ok(File {
        file_name: dfs_path.file_name,
        directory: dfs_path.directory,
        locked: false,
        load_address: 0.try_into()?,
        execution_address: 0.try_into()?,
        content_path,
        r#type: file_type,
    })
}
