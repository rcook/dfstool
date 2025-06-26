use crate::boot::Boot;
use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use crate::file_descriptor::FileDescriptor;
use anyhow::Result;
use std::io::Read;

#[derive(Debug)]
pub struct Catalogue {
    _bytes: CatalogueBytes,
    title: String,
    cycle_number: u8,
    number: u8,
    _offset: u8,
    boot: Boot,
    sectors: u16,
    pub file_descriptors: Vec<FileDescriptor>,
}

impl Catalogue {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut bytes = [0; SECTOR_SIZE * 2];
        reader.read_exact(&mut bytes)?;
        let title = Self::title(&bytes)?;
        let cycle_number = Self::cycle_number(&bytes);
        let (number, offset) = Self::file_number_and_offset(&bytes);
        let boot = Self::boot(&bytes);
        let sectors = Self::sectors(&bytes);
        let file_descriptors = Self::file_descriptors(&bytes, number)?;
        Ok(Self {
            _bytes: bytes,
            title,
            cycle_number,
            number,
            _offset: offset,
            boot,
            sectors,
            file_descriptors,
        })
    }

    pub fn show(&self) {
        println!("Title: \"{}\"", self.title);
        println!("Cycle number: {}", self.cycle_number);
        println!("File number: {}", self.number);
        println!("Boot: {:?}", self.boot);
        println!("Sectors: {:?}", self.sectors);
        println!("Files:");
        for file_descriptor in &self.file_descriptors {
            if file_descriptor.locked {
                println!(
                    "  {}.{:<10} {:06X} {:06X} {:06X} {:04X} (locked)",
                    file_descriptor.directory,
                    file_descriptor.file_name,
                    file_descriptor.load_address,
                    file_descriptor.execution_address,
                    file_descriptor.length,
                    file_descriptor.start_sector
                )
            } else {
                println!(
                    "  {}.{:<10} {:06X} {:06X} {:06X} {:04X}",
                    file_descriptor.directory,
                    file_descriptor.file_name,
                    file_descriptor.load_address,
                    file_descriptor.execution_address,
                    file_descriptor.length,
                    file_descriptor.start_sector
                )
            }
        }
    }

    fn is_disc_title_char(c: char) -> bool {
        c == '\0' || !c.is_ascii_control()
    }

    fn title(bytes: &CatalogueBytes) -> Result<String> {
        let mut title = String::with_capacity(12);
        title.push_str(str::from_utf8(&bytes[0..8])?);
        title.push_str(str::from_utf8(&bytes[SECTOR_SIZE..SECTOR_SIZE + 4])?);
        assert!(title.chars().all(Self::is_disc_title_char));
        let s = title.trim_end_matches(' ').trim_end_matches('\0');
        Ok(String::from(s))
    }

    fn cycle_number(bytes: &CatalogueBytes) -> u8 {
        let bcd = bytes[SECTOR_SIZE + 4];
        let value = ((bcd >> 4) * 10) + (bcd & 0b00001111);
        value
    }

    fn file_number_and_offset(bytes: &CatalogueBytes) -> (u8, u8) {
        let offset = bytes[SECTOR_SIZE + 5];
        assert_eq!(0, offset & 0b00000111);
        let number = offset >> 3;
        (number, offset)
    }

    fn boot(bytes: &CatalogueBytes) -> Boot {
        let temp = bytes[SECTOR_SIZE + 6];
        assert_eq!(0, temp & 0b11001100);
        Boot::from_byte(temp)
    }

    fn sectors(bytes: &CatalogueBytes) -> u16 {
        let lo_bits = bytes[SECTOR_SIZE + 7];
        let temp = bytes[SECTOR_SIZE + 6];
        assert_eq!(0, temp & 0b11001100);
        let hi_bits = ((temp & 0b00000011) as u16) << 8;
        let sectors = hi_bits + lo_bits as u16;
        assert!(sectors >= 2 && sectors <= 1023);
        sectors
    }

    fn file_descriptors(bytes: &CatalogueBytes, number: u8) -> Result<Vec<FileDescriptor>> {
        (0..number)
            .map(|i| FileDescriptor::from_catalogue_bytes(bytes, i as usize))
            .collect()
    }
}
