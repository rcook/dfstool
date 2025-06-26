use crate::catalogue::Catalogue;
use anyhow::Result;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn run() -> Result<()> {
    let mut file = File::open("test.ssd")?;
    let catalogue = Catalogue::read(&mut file)?;
    catalogue.show();

    for file_descriptor in &catalogue.file_descriptors {
        println!(
            "{}.{}:",
            file_descriptor.directory, file_descriptor.file_name
        );
        let mut bytes = vec![0; file_descriptor.length as usize];
        file.seek(SeekFrom::Start(file_descriptor.start_sector as u64 * 256))?;
        file.read_exact(&mut bytes)?;

        let s = str::from_utf8(&bytes)?;
        let s = s.replace('\r', "\n");
        println!("[{s}]")
    }

    Ok(())
}
