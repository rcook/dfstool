use crate::bbc_basic::{detokenize_source, is_bbc_basic_file};
use crate::catalogue::Catalogue;
use crate::constants::{LOSSLESS_BBC_BASIC_EXT, LOSSY_BBC_BASIC_EXT, MANIFEST_VERSION};
use crate::disc_side::DISC_SIDE_0;
use crate::file_type::{FileType, KnownFileType};
use crate::manifest::Manifest;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use std::ffi::OsStr;
use std::fs::{File, create_dir_all};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub fn do_extract(
    input_path: &Path,
    output_dir: &Path,
    overwrite: bool,
    detokenize: bool,
    lossless: bool,
) -> Result<()> {
    let mut input_file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => bail!(
            "input file {input_path} not found",
            input_path = input_path.display()
        ),
        Err(e) => bail!(e),
    };
    let catalogue = Catalogue::from_reader(&mut input_file, *DISC_SIDE_0)?;

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

    let files = catalogue
        .entries
        .into_iter()
        .map(|entry| {
            let d = &entry.descriptor;

            let mut bytes = vec![0; u32::from(entry.length) as usize];
            input_file.seek(SeekFrom::Start(
                u64::from(u16::from(entry.start_sector)) * 256,
            ))?;
            input_file.read_exact(&mut bytes)?;

            let content_path = output_dir.join(d.content_path());
            let mut content_file = open_for_write(&content_path, overwrite)?;
            content_file.write_all(&bytes)?;

            let is_bbc_basic = is_bbc_basic_file(&content_path)?;
            if detokenize && is_bbc_basic {
                // Attempt to detokenize the file just in case it contains BASIC
                // Don't fail if it can't be detokenized
                _ = detokenize_file(&content_path, overwrite, lossless);
            }

            Ok(d.to_manifest_file(FileType::Known(if is_bbc_basic {
                KnownFileType::BbcBasic
            } else {
                KnownFileType::Other
            })))
        })
        .collect::<Result<Vec<_>>>()?;

    let manifest_file = open_for_write(&manifest_path, overwrite)?;
    serde_json::to_writer_pretty(
        manifest_file,
        &Manifest {
            version: Some(MANIFEST_VERSION),
            disc_title: Some(catalogue.disc_title),
            disc_size: catalogue.disc_size,
            files,
        },
    )?;

    Ok(())
}

fn detokenize_file(input_path: &Path, overwrite: bool, lossless: bool) -> Result<()> {
    let output_dir = input_path
        .parent()
        .ok_or_else(|| anyhow!("cannot get parent"))?;
    let file_name = input_path
        .file_name()
        .ok_or_else(|| anyhow!("cannot get file name"))?;
    let output_path = output_dir.join(if lossless {
        format!("{f}{LOSSLESS_BBC_BASIC_EXT}", f = file_name.display())
    } else {
        format!("{f}{LOSSY_BBC_BASIC_EXT}", f = file_name.display())
    });

    let output_file = open_for_write(&output_path, overwrite)?;
    let mut input_file = File::open(input_path)?;
    let mut bytes = Vec::new();
    input_file.read_to_end(&mut bytes)?;
    detokenize_source(output_file, &bytes, lossless)
}
