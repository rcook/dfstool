use crate::bbc_basic::{detokenize_source, is_bbc_basic_file};
use crate::catalogue::Catalogue;
use crate::manifest::Manifest;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use std::ffi::OsStr;
use std::fs::{File, create_dir_all, remove_file};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub fn do_extract(
    input_path: &Path,
    output_dir: &Path,
    overwrite: bool,
    detokenize: bool,
) -> Result<()> {
    let mut input_file = File::open(input_path)?;
    let catalogue = Catalogue::from_reader(&mut input_file)?;

    if !output_dir.exists() {
        create_dir_all(output_dir)?;
    }

    let mut manifest_file_name = String::new();
    manifest_file_name.push_str(input_path.file_name().and_then(OsStr::to_str).ok_or_else(
        || {
            anyhow!(
                "could not get file name from {input_path}",
                input_path = input_path.display()
            )
        },
    )?);
    manifest_file_name.push_str(".json");
    let manifest_path = output_dir.join(manifest_file_name);

    let mut manifest_files = Vec::with_capacity(catalogue.entries.len());
    for entry in &catalogue.entries {
        let d = &entry.descriptor;

        let mut bytes = vec![0; entry.length.as_usize()];
        input_file.seek(SeekFrom::Start(entry.start_sector.as_u64() * 256))?;
        input_file.read_exact(&mut bytes)?;

        let manifest_file = d.to_manifest_file();
        let content_path = output_dir.join(&manifest_file.content_path);
        let mut content_file = open_for_write(&content_path, overwrite)?;
        content_file.write_all(&bytes)?;

        if detokenize && is_bbc_basic_file(&content_path, d)? {
            // Attempt to detokenize the file just in case it contains BASIC
            // Don't fail if it can't be detokenized
            _ = detokenize_file(&content_path, overwrite)
        }

        manifest_files.push(manifest_file);
    }

    let manifest_file = open_for_write(&manifest_path, overwrite)?;
    serde_json::to_writer_pretty(
        manifest_file,
        &Manifest {
            files: manifest_files,
        },
    )?;

    Ok(())
}

fn detokenize_file(input_path: &Path, overwrite: bool) -> Result<()> {
    let output_dir = input_path
        .parent()
        .ok_or_else(|| anyhow!("cannot get parent"))?;
    let file_name = input_path
        .file_name()
        .ok_or_else(|| anyhow!("cannot get file name"))?;
    let output_path = output_dir.join(format!("{f}.bas", f = file_name.display()));

    let output_file = open_for_write(&output_path, overwrite)?;
    let mut input_file = File::open(input_path)?;
    let mut bytes = Vec::new();
    input_file.read_to_end(&mut bytes)?;
    match detokenize_source(output_file, &bytes) {
        Ok(()) => Ok(()),
        Err(e) => {
            remove_file(&output_path)?;
            bail!(e)
        }
    }
}
