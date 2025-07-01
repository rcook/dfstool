use crate::dfs::{
    BootOption, CatalogueBytes, CatalogueEntry, CycleNumber, DiscSize, DiscTitle, FileOffset,
    SECTOR_BYTES, SectorSize,
};
use crate::dsd_reader::DsdReader;
use crate::image_reader::ImageReader;
use crate::ssd_reader::SsdReader;
use anyhow::{Result, bail};
use std::ffi::OsStr;
use std::fs::File;
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
    pub fn from_image_file(path: &Path) -> Result<Catalogue> {
        let f = File::open(path)?;
        match path.extension().and_then(OsStr::to_str) {
            Some("dsd") => {
                let mut reader = DsdReader::new(f, SECTOR_BYTES);
                Self::from_image_reader(&mut reader)
            }
            Some("ssd") => {
                let mut reader = SsdReader::new(f, SECTOR_BYTES);
                Self::from_image_reader(&mut reader)
            }
            _ => bail!("unsupported file type {path}", path = path.display()),
        }
    }

    pub fn from_image_reader<R: ImageReader>(reader: &mut R) -> Result<Catalogue> {
        let sector_bytes = usize::from(reader.sector_bytes());
        let mut bytes = vec![0; sector_bytes * 2];
        reader.read_bytes(0, SectorSize::ZERO, &mut bytes[0..sector_bytes])?;
        reader.read_bytes(
            0,
            SectorSize::ONE,
            &mut bytes[sector_bytes..sector_bytes * 2],
        )?;

        if !Self::is_valid_catalogue(&bytes) {
            bail!("input file does not contain a valid disc image")
        }

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

    // https://www.geraldholdsworth.co.uk/documents/DiscImage.pdf
    fn is_valid_catalogue(bytes: &CatalogueBytes) -> bool {
        if !bytes[0x0000..0x0009]
            .iter()
            .all(|&b| (b & 0x80) == 0 && b > 31 || b == 0)
        {
            return false;
        }

        if !bytes[0x0100..0x0104]
            .iter()
            .all(|&b| (b & 0x80) == 0 && b > 31 || b == 0)
        {
            return false;
        }

        if (bytes[0x0105] & 0b0000_0111) != 0 {
            return false;
        }
        if (bytes[0x0106] & 0b1100_1100) != 0 {
            return false;
        }

        true
    }
}
