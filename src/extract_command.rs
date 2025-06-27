use crate::catalogue::Catalogue;
use crate::constants::{SSD_CONTENT_FILE_EXT, SSD_METADATA_FILE_EXT};
use crate::detokenize::detokenize_source;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use path_absolutize::Absolutize;
use std::fs::{File, remove_file};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub fn do_extract(ssd_path: &Path, overwrite: bool) -> Result<()> {
    let mut ssd_file = File::open(ssd_path)?;
    let catalogue = Catalogue::from_reader(&mut ssd_file)?;

    for entry in &catalogue.entries {
        let d = &entry.descriptor;

        let mut bytes = vec![0; entry.length.as_usize()];
        ssd_file.seek(SeekFrom::Start(entry.start_sector.as_u64() * 256))?;
        ssd_file.read_exact(&mut bytes)?;

        let content_path = PathBuf::from(&format!(
            "{}_{}.{ext}",
            d.directory,
            d.file_name,
            ext = SSD_CONTENT_FILE_EXT
        ))
        .absolutize()?
        .to_path_buf();
        let mut f = open_for_write(&content_path, overwrite)?;
        f.write_all(&bytes)?;

        let metadata_path = PathBuf::from(&format!(
            "{}_{}.{ext}",
            d.directory,
            d.file_name,
            ext = SSD_METADATA_FILE_EXT
        ))
        .absolutize()?
        .to_path_buf();
        let f = if overwrite {
            File::create(metadata_path)?
        } else {
            File::create_new(metadata_path)?
        };
        serde_json::to_writer_pretty(f, d)?;

        // Attempt to detokenize the file just in case it contains BASIC
        // Don't fail if it can't be detokenized
        _ = detokenize_file(&content_path, overwrite)
    }

    Ok(())
}

fn detokenize_file(input_path: &Path, overwrite: bool) -> Result<()> {
    let dir = input_path
        .parent()
        .ok_or_else(|| anyhow!("cannot get parent"))?;
    let file_name = input_path
        .file_name()
        .ok_or_else(|| anyhow!("cannot get file name"))?;

    let output_path = dir.join(format!("{f}.bas", f = file_name.display()));
    let output_file = open_for_write(&output_path, overwrite)?;
    let mut f = File::open(input_path)?;
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)?;
    match detokenize_source(output_file, &bytes) {
        Ok(()) => Ok(()),
        Err(e) => {
            remove_file(&output_path)?;
            bail!(e)
        }
    }
}
