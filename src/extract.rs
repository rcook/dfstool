use crate::catalogue::Catalogue;
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

        let mut bytes = vec![0; d.length.as_usize()];
        ssd_file.seek(SeekFrom::Start(entry.start_sector.as_u64() * 256))?;
        ssd_file.read_exact(&mut bytes)?;

        let local_file_name = format!("{}_{}.ssdfile", d.directory, d.file_name);
        let mut local_file = open_for_write(local_file_name, overwrite)?;
        local_file.write_all(&bytes)?;

        let local_metadata_file_name = format!("{}_{}.ssdfile.json", d.directory, d.file_name);
        let local_metadata_file = if overwrite {
            File::create(local_metadata_file_name)?
        } else {
            File::create_new(local_metadata_file_name)?
        };
        serde_json::to_writer_pretty(local_metadata_file, d)?;
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
