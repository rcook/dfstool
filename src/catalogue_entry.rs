use crate::catalogue_bytes::CatalogueBytes;
use crate::constants::SECTOR_SIZE;
use crate::file_descriptor::FileDescriptor;
use crate::length::Length;
use crate::start_sector::StartSector;
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

    pub fn write_to_catalogue(bytes: &mut [u8], entries: &[Self]) -> Result<()> {
        for (index, entry) in entries.iter().enumerate() {
            entry.write_to_inner(bytes, index)?;
        }
        Ok(())
    }

    pub const fn new(
        descriptor: FileDescriptor,
        length: Length,
        start_sector: StartSector,
    ) -> Self {
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
        let locked = (temp & 0b1000_0000) != 0;
        let d = (temp & 0b0111_1111) as char;
        let directory = d.try_into()?;

        let offset2 = SECTOR_SIZE + offset;

        let extra_bits = bytes[offset2 + 6];

        let load_address = (u32::from(bytes[offset2])
            + (u32::from(bytes[offset2 + 1]) << 8)
            + (u32::from((extra_bits & 0b0000_1100) >> 2) << 16))
            .try_into()?;
        let execution_address = (u32::from(bytes[offset2 + 2])
            + (u32::from(bytes[offset2 + 3]) << 8)
            + (u32::from((extra_bits & 0b1100_0000) >> 6) << 16))
            .try_into()?;
        let length = (u32::from(bytes[offset2 + 4])
            + (u32::from(bytes[offset2 + 5]) << 8)
            + (u32::from((extra_bits & 0b0011_0000) >> 4) << 16))
            .try_into()?;
        let start_sector = (u16::from(bytes[offset2 + 7])
            + (u16::from(extra_bits & 0b0000_0011) << 8))
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
        bytes[offset + len..offset + 7].fill(32);
        bytes[offset + 7] = (if self.descriptor.locked { 0x80 } else { 0 })
            | self.descriptor.directory.as_char() as u8;

        let offset2 = offset + SECTOR_SIZE;

        let load_address = self.descriptor.load_address.as_u32();
        let execution_address = self.descriptor.execution_address.as_u32();
        let length = self.length.as_u32();
        let start_sector = self.start_sector.as_u16();

        bytes[offset2] = u8::try_from(load_address & 0xff)?;
        bytes[offset2 + 1] = u8::try_from((load_address >> 8) & 0xff)?;
        bytes[offset2 + 2] = u8::try_from(execution_address & 0xff)?;
        bytes[offset2 + 3] = u8::try_from((execution_address >> 8) & 0xff)?;
        bytes[offset2 + 4] = u8::try_from(length & 0xff)?;
        bytes[offset2 + 5] = u8::try_from((length >> 8) & 0xff)?;
        bytes[offset2 + 7] = u8::try_from(start_sector & 0xff)?;
        bytes[offset2 + 6] =
            Self::make_extra_bits(load_address, execution_address, length, start_sector)?;
        Ok(())
    }

    fn make_extra_bits(
        load_address: u32,
        execution_address: u32,
        length: u32,
        start_sector: u16,
    ) -> Result<u8> {
        Ok(u8::try_from(
            (load_address >> 16)
                << (2 + (execution_address >> 16))
                << (6 + (length >> 16))
                << (4 + (start_sector >> 8)),
        )?)
    }
}
