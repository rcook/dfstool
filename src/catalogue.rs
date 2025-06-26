use crate::boot_option::BootOption;
use crate::catalogue_bytes::CatalogueBytes;
use crate::catalogue_entry::CatalogueEntry;
use crate::constants::SECTOR_SIZE;
use crate::cycle_number::CycleNumber;
use crate::disc_title::DiscTitle;
use crate::file_offset::FileOffset;
use crate::u10::DiscSize;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// https://beebwiki.mdfs.net/Acorn_DFS_disc_format
#[derive(Debug)]
pub struct Catalogue {
    disc_title: DiscTitle,
    cycle_number: CycleNumber,
    file_offset: FileOffset,
    boot_option: BootOption,
    disc_size: DiscSize,
    pub entries: Vec<CatalogueEntry>,
}

impl Catalogue {
    pub fn from_file(ssd_path: &Path) -> Result<Catalogue> {
        Self::from_reader(File::open(ssd_path)?)
    }

    pub fn from_reader<R: Read>(mut reader: R) -> Result<Catalogue> {
        let mut bytes = [0; SECTOR_SIZE * 2];
        reader.read_exact(&mut bytes)?;
        Self::from_catalogue_bytes(&bytes)
    }

    pub fn from_catalogue_bytes(bytes: &CatalogueBytes) -> Result<Self> {
        let disc_title = DiscTitle::from_catalogue_bytes(bytes)?;
        let cycle_number = CycleNumber::from_catalogue_bytes(bytes)?;
        let file_offset = FileOffset::from_catalogue_bytes(bytes)?;
        let boot_option = BootOption::from_catalogue_bytes(bytes)?;
        let disc_size = DiscSize::from_catalogue_bytes(bytes)?;
        let entries = CatalogueEntry::from_catalogue_bytes(bytes, file_offset.number())?;
        Ok(Self::new(
            disc_title,
            cycle_number,
            file_offset,
            boot_option,
            disc_size,
            entries,
        ))
    }

    pub fn new(
        disc_title: DiscTitle,
        cycle_number: CycleNumber,
        file_offset: FileOffset,
        boot_option: BootOption,
        disc_size: DiscSize,
        entries: Vec<CatalogueEntry>,
    ) -> Self {
        Self {
            disc_title,
            cycle_number,
            file_offset,
            boot_option,
            disc_size,
            entries,
        }
    }

    pub fn show(&self) {
        println!("Title: \"{}\"", self.disc_title);
        println!("Cycle number: {}", self.cycle_number);
        println!("File number: {}", self.file_offset.number());
        println!("Boot: {:?}", self.boot_option);
        println!("Sectors: {:?}", self.disc_size.as_u64());
        println!("Files:");
        for entry in &self.entries {
            let d = &entry.descriptor;

            let extra = String::from(if d.locked { " (locked)" } else { "" });
            println!(
                "  {directory}.{file_name:<10} {load_address} {execution_address} {length} {start_sector}{extra}",
                directory = d.directory,
                file_name = d.file_name.to_string(),
                load_address = d.load_address,
                execution_address = d.execution_address,
                length = entry.length,
                start_sector = entry.start_sector,
                extra = extra
            )
        }
    }
}
