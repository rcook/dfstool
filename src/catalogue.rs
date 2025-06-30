use crate::boot_option::BootOption;
use crate::catalogue_bytes::CatalogueBytes;
use crate::catalogue_entry::CatalogueEntry;
use crate::constants::SECTOR_SIZE;
use crate::cycle_number::CycleNumber;
use crate::disc_size::DiscSize;
use crate::disc_title::DiscTitle;
use crate::file_offset::FileOffset;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// https://beebwiki.mdfs.net/Acorn_DFS_disc_format
#[derive(Debug)]
pub struct Catalogue {
    pub disc_title: DiscTitle,
    pub cycle_number: CycleNumber,
    pub file_offset: FileOffset,
    pub boot_option: BootOption,
    pub disc_size: DiscSize,
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

    #[allow(clippy::similar_names)]
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

    pub const fn new(
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

    pub fn write_to_catalogue(&self, bytes: &mut [u8]) -> Result<()> {
        self.disc_title.write_to_catalogue(bytes);
        self.cycle_number.write_to_catalogue(bytes)?;
        self.file_offset.write_to_catalogue(bytes);
        self.boot_option.write_to_catalogue(bytes);
        self.disc_size.write_to_catalogue(bytes);
        CatalogueEntry::write_to_catalogue(bytes, &self.entries)?;
        Ok(())
    }
}
