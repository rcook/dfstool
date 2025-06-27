use crate::catalogue::Catalogue;
use crate::constants::{SSD_CONTENT_FILE_EXT, SSD_METADATA_FILE_EXT};
use crate::detokenize::detokenize_source;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
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

    for entry in &catalogue.entries {
        let d = &entry.descriptor;

        let mut bytes = vec![0; entry.length.as_usize()];
        input_file.seek(SeekFrom::Start(entry.start_sector.as_u64() * 256))?;
        input_file.read_exact(&mut bytes)?;

        let content_path = output_dir.join(format!(
            "{}_{}.{ext}",
            d.directory,
            d.file_name,
            ext = SSD_CONTENT_FILE_EXT
        ));
        let mut output_file = open_for_write(&content_path, overwrite)?;
        output_file.write_all(&bytes)?;

        let metadata_path = output_dir.join(format!(
            "{}_{}.{ext}",
            d.directory,
            d.file_name,
            ext = SSD_METADATA_FILE_EXT
        ));
        let output_file = open_for_write(&metadata_path, overwrite)?;
        serde_json::to_writer_pretty(output_file, d)?;

        if detokenize {
            // Attempt to detokenize the file just in case it contains BASIC
            // Don't fail if it can't be detokenized
            _ = detokenize_file(&content_path, overwrite)
        }
    }

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
