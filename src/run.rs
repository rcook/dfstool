use crate::catalogue::Catalogue;
use anyhow::Result;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn run() -> Result<()> {
    let mut file = File::open("test.ssd")?;
    let catalogue = Catalogue::read(&mut file)?;
    catalogue.show();

    for entry in &catalogue.entries {
        let d = &entry.descriptor;
        println!("{}.{}:", d.directory, d.file_name);
        let mut bytes = vec![0; d.length.as_usize()];
        file.seek(SeekFrom::Start(entry.start_sector.as_u64() * 256))?;
        file.read_exact(&mut bytes)?;

        let s = str::from_utf8(&bytes)?;
        let s = s.replace('\r', "\n");
        println!("[{s}]")
    }

    Ok(())
}
