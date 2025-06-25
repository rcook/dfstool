use anyhow::Result;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const SECTOR_SIZE: usize = 256;
type CatalogueBytes = [u8; SECTOR_SIZE * 2];

#[derive(Debug)]
#[repr(u8)]
enum Boot {
    None = 0,
    Load = 1,
    Run = 2,
    Exec = 3,
}

impl Boot {
    fn from_byte(byte: u8) -> Self {
        match (byte & 0b00110000) >> 4 {
            0 => Self::None,
            1 => Self::Load,
            2 => Self::Run,
            3 => Self::Exec,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct FileDescriptor {
    file_name: String,
    directory: char,
    locked: bool,
    load_address: u32,
    execution_address: u32,
    length: u32,
    start_sector: u16,
}

impl FileDescriptor {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes, index: usize) -> Result<Self> {
        let offset = ((index + 1) * 8) as usize;
        let file_name_bytes = &bytes[offset..offset + 7];
        let file_name = str::from_utf8(file_name_bytes)?.trim_end_matches(' ');
        assert!(file_name.chars().all(Self::is_file_name_char));
        let temp = bytes[offset + 7];
        let locked = (temp & 0b10000000) != 0;
        let d = (temp & 0b01111111) as char;
        assert!(Self::is_file_name_char(d));

        let extra_bits = bytes[SECTOR_SIZE + offset + 6];

        let load_address = bytes[SECTOR_SIZE + offset] as u32
            + ((bytes[SECTOR_SIZE + offset + 1] as u32) << 8)
            + ((((extra_bits & 0b00001100) >> 2) as u32) << 16);
        let execution_address = bytes[SECTOR_SIZE + offset + 2] as u32
            + ((bytes[SECTOR_SIZE + offset + 3] as u32) << 8)
            + ((((extra_bits & 0b11000000) >> 6) as u32) << 16);
        let length = bytes[SECTOR_SIZE + offset + 4] as u32
            + ((bytes[SECTOR_SIZE + offset + 5] as u32) << 8)
            + ((((extra_bits & 0b00110000) >> 4) as u32) << 16);
        let start_sector =
            bytes[SECTOR_SIZE + offset + 7] as u16 + (((extra_bits & 0b00000011) as u16) << 8);

        Ok(Self {
            file_name: file_name.to_string(),
            directory: d,
            locked,
            load_address,
            execution_address,
            length,
            start_sector,
        })
    }

    fn is_file_name_char(c: char) -> bool {
        const INVALID_CHARS: &str = ".:\"#* ";
        let value = c as u8;
        if value < 0x20 || value > 0x7e {
            false
        } else {
            !INVALID_CHARS.contains(c)
        }
    }
}

#[derive(Debug)]
struct Catalogue {
    _bytes: CatalogueBytes,
    title: String,
    cycle_number: u8,
    number: u8,
    _offset: u8,
    boot: Boot,
    sectors: u16,
    file_descriptors: Vec<FileDescriptor>,
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

fn main() -> Result<()> {
    let mut file = File::open("test.ssd")?;
    let catalogue = Catalogue::read(&mut file)?;
    show_catalogue(&catalogue);

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

fn show_catalogue(catalogue: &Catalogue) {
    println!("Title: \"{}\"", catalogue.title);
    println!("Cycle number: {}", catalogue.cycle_number);
    println!("File number: {}", catalogue.number);
    println!("Boot: {:?}", catalogue.boot);
    println!("Sectors: {:?}", catalogue.sectors);
    println!("Files:");
    for file_descriptor in &catalogue.file_descriptors {
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
