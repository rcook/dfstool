use crate::catalogue::Catalogue;
use anyhow::Result;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
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

        let mut local_file = if overwrite {
            File::create(local_file_name)?
        } else {
            File::create_new(local_file_name)?
        };
        local_file.write_all(&bytes)?;
    }

    Ok(())
}
