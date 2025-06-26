use crate::bcd_value::BcdValue;
use crate::boot_option::BootOption;
use crate::catalogue_bytes::CatalogueBytes;
use crate::catalogue_entry::CatalogueEntry;
use crate::constants::SECTOR_SIZE;
use crate::disc_title::DiscTitle;
use crate::file_offset::FileOffset;
use crate::u10::DiscSize;
use anyhow::Result;
use std::io::Read;

#[derive(Debug)]
pub struct Catalogue {
    disc_title: DiscTitle,
    cycle_number: BcdValue,
    file_offset: FileOffset,
    boot_option: BootOption,
    disc_size: DiscSize,
    pub entries: Vec<CatalogueEntry>,
}

impl Catalogue {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut bytes = [0; SECTOR_SIZE * 2];
        reader.read_exact(&mut bytes)?;
        let disc_title = Self::disc_title(&bytes)?;
        let cycle_number = Self::cycle_number(&bytes)?;
        let file_offset = Self::file_offset(&bytes)?;
        let boot_option = Self::boot(&bytes)?;
        let disc_size = Self::disc_size(&bytes)?;
        let entries = Self::entries(&bytes, file_offset.number())?;
        Ok(Self {
            disc_title,
            cycle_number,
            file_offset,
            boot_option,
            disc_size,
            entries,
        })
    }

    pub fn show(&self) {
        println!("Title: \"{}\"", self.disc_title);
        println!("Cycle number: {}", self.cycle_number);
        println!("File number: {}", self.file_offset.number());
        println!("Boot: {:?}", self.boot_option);
        println!("Sectors: {:?}", self.disc_size);
        println!("Files:");
        for entry in &self.entries {
            let d = &entry.descriptor;
            if d.locked {
                println!(
                    "  {directory}.{file_name:<10} {load_address} {execution_address} {length} {start_sector} (locked)",
                    directory = d.directory,
                    file_name = d.file_name,
                    load_address = d.load_address,
                    execution_address = d.execution_address,
                    length = d.length,
                    start_sector = entry.start_sector
                )
            } else {
                println!(
                    "  {directory}.{file_name:<10} {load_address} {execution_address} {length} {start_sector}",
                    directory = d.directory,
                    file_name = d.file_name,
                    load_address = d.load_address,
                    execution_address = d.execution_address,
                    length = d.length,
                    start_sector = entry.start_sector
                )
            }
        }
    }

    fn disc_title(bytes: &CatalogueBytes) -> Result<DiscTitle> {
        let mut title = String::with_capacity(12);
        title.push_str(str::from_utf8(&bytes[0..8])?);
        title.push_str(str::from_utf8(&bytes[SECTOR_SIZE..SECTOR_SIZE + 4])?);
        let s = title.trim_end_matches(' ').trim_end_matches('\0');
        s.parse()
    }

    fn cycle_number(bytes: &CatalogueBytes) -> Result<BcdValue> {
        bytes[SECTOR_SIZE + 4].try_into()
    }

    fn file_offset(bytes: &CatalogueBytes) -> Result<FileOffset> {
        let offset = bytes[SECTOR_SIZE + 5];
        offset.try_into()
    }

    fn boot(bytes: &CatalogueBytes) -> Result<BootOption> {
        let temp = bytes[SECTOR_SIZE + 6];
        assert_eq!(0, temp & 0b11001100);
        temp.try_into()
    }

    fn disc_size(bytes: &CatalogueBytes) -> Result<DiscSize> {
        let lo_bits = bytes[SECTOR_SIZE + 7];
        let temp = bytes[SECTOR_SIZE + 6];
        assert_eq!(0, temp & 0b11001100);
        let hi_bits = ((temp & 0b00000011) as u16) << 8;
        let disc_size = hi_bits + lo_bits as u16;
        assert!((2..=1023).contains(&disc_size));
        disc_size.try_into()
    }

    fn entries(bytes: &CatalogueBytes, number: u8) -> Result<Vec<CatalogueEntry>> {
        (0..number)
            .map(|i| CatalogueEntry::from_catalogue_bytes(bytes, i as usize))
            .collect()
    }
}
