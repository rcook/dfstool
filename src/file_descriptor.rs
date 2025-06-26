use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use anyhow::Result;

#[derive(Debug)]
pub struct FileDescriptor {
    pub file_name: String,
    pub directory: char,
    pub locked: bool,
    pub load_address: u32,
    pub execution_address: u32,
    pub length: u32,
    pub start_sector: u16,
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
