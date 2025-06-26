use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use crate::file_descriptor::FileDescriptor;
use crate::u10::StartSector;
use crate::u18::Length;
use anyhow::Result;

#[derive(Debug)]
pub struct CatalogueEntry {
    pub descriptor: FileDescriptor,
    pub length: Length,
    pub start_sector: StartSector,
}

impl CatalogueEntry {
    pub fn from_catalogue_bytes(bytes: &CatalogueBytes, number: u8) -> Result<Vec<Self>> {
        (0..number)
            .map(|i| Self::from_catalogue_bytes_inner(bytes, i as usize))
            .collect()
    }

    pub fn write_to(bytes: &mut [u8], entries: &[Self]) -> Result<()> {
        for (index, entry) in entries.iter().enumerate() {
            entry.write_to_inner(bytes, index)?
        }
        Ok(())
    }

    pub fn new(descriptor: FileDescriptor, length: Length, start_sector: StartSector) -> Self {
        Self {
            descriptor,
            length,
            start_sector,
        }
    }

    fn from_catalogue_bytes_inner(bytes: &CatalogueBytes, index: usize) -> Result<Self> {
        let offset = (index + 1) * 8;
        let file_name_bytes = &bytes[offset..offset + 7];
        let file_name_str = str::from_utf8(file_name_bytes)?.trim_end_matches(['\0', ' ']);
        let file_name = file_name_str.parse()?;
        let temp = bytes[offset + 7];
        let locked = (temp & 0b10000000) != 0;
        let d = (temp & 0b01111111) as char;
        let directory = d.try_into()?;

        let extra_bits = bytes[SECTOR_SIZE + offset + 6];

        let load_address = (bytes[SECTOR_SIZE + offset] as u32
            + ((bytes[SECTOR_SIZE + offset + 1] as u32) << 8)
            + ((((extra_bits & 0b00001100) >> 2) as u32) << 16))
            .try_into()?;
        let execution_address = (bytes[SECTOR_SIZE + offset + 2] as u32
            + ((bytes[SECTOR_SIZE + offset + 3] as u32) << 8)
            + ((((extra_bits & 0b11000000) >> 6) as u32) << 16))
            .try_into()?;
        let length = (bytes[SECTOR_SIZE + offset + 4] as u32
            + ((bytes[SECTOR_SIZE + offset + 5] as u32) << 8)
            + ((((extra_bits & 0b00110000) >> 4) as u32) << 16))
            .try_into()?;
        let start_sector = (bytes[SECTOR_SIZE + offset + 7] as u16
            + (((extra_bits & 0b00000011) as u16) << 8))
            .try_into()?;

        Ok(Self::new(
            FileDescriptor::new(
                file_name,
                directory,
                locked,
                load_address,
                execution_address,
            ),
            length,
            start_sector,
        ))
    }

    fn write_to_inner(&self, bytes: &mut [u8], index: usize) -> Result<()> {
        let offset = (index + 1) * 8;
        let s = self.descriptor.file_name.as_str();
        let len = s.len();
        bytes[offset..offset + len].copy_from_slice(s.as_bytes());
        bytes[offset + 7] = (if self.descriptor.locked { 0x80 } else { 0 })
            | self.descriptor.directory.as_char() as u8;
        Ok(())
    }
}
