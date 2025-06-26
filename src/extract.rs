use crate::catalogue::Catalogue;
use crate::constants::{SSD_CONTENT_FILE_EXT, SSD_METADATA_FILE_EXT};
use anyhow::{Result, bail};
use std::fmt::Display;
use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub fn do_extract(ssd_path: &Path, overwrite: bool) -> Result<()> {
    let mut ssd_file = File::open(ssd_path)?;
    let catalogue = Catalogue::read(&mut ssd_file)?;

    for entry in &catalogue.entries {
        let d = &entry.descriptor;

        let mut bytes = vec![0; entry.length.as_usize()];
        ssd_file.seek(SeekFrom::Start(entry.start_sector.as_u64() * 256))?;
        ssd_file.read_exact(&mut bytes)?;

        let content_file_name = format!(
            "{}_{}.{ext}",
            d.directory,
            d.file_name,
            ext = SSD_CONTENT_FILE_EXT
        );
        let mut f = open_for_write(content_file_name, overwrite)?;
        f.write_all(&bytes)?;

        let metadata_file_name = format!(
            "{}_{}.{ext}",
            d.directory,
            d.file_name,
            ext = SSD_METADATA_FILE_EXT
        );
        let f = if overwrite {
            File::create(metadata_file_name)?
        } else {
            File::create_new(metadata_file_name)?
        };
        serde_json::to_writer_pretty(f, d)?;
    }

    Ok(())
}

fn open_for_write<P: Display + AsRef<Path>>(path: P, overwrite: bool) -> Result<File> {
    let result = if overwrite {
        File::create(&path)
    } else {
        File::create_new(&path)
    };

    match result {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            bail!("output file {path} already exists: pass --overwrite to overwrite")
        }
        Err(e) => bail!(e),
    }
}
