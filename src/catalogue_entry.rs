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

        let (load_address_top, execution_address_top, length_top, start_sector_top) =
            Self::extract_extra_bits(bytes[offset2 + 6]);

        let load_address =
            (u32::from(bytes[offset2]) + (u32::from(bytes[offset2 + 1]) << 8) + load_address_top)
                .try_into()?;
        let execution_address = (u32::from(bytes[offset2 + 2])
            + (u32::from(bytes[offset2 + 3]) << 8)
            + execution_address_top)
            .try_into()?;
        let length =
            (u32::from(bytes[offset2 + 4]) + (u32::from(bytes[offset2 + 5]) << 8) + length_top)
                .try_into()?;
        let start_sector = (u16::from(bytes[offset2 + 7]) + start_sector_top).try_into()?;

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
            | self.descriptor.directory.to_char() as u8;

        let offset2 = offset + SECTOR_SIZE;

        let load_address = u32::from(self.descriptor.load_address);
        let execution_address = u32::from(self.descriptor.execution_address);
        let length = u32::from(self.length);
        let start_sector = u16::from(self.start_sector);

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

    fn extract_extra_bits(value: u8) -> (u32, u32, u32, u16) {
        let load_address_top = u32::from((value & 0b0000_1100) >> 2) << 16;
        let execution_address_top = u32::from((value & 0b1100_0000) >> 6) << 16;
        let length_top = u32::from((value & 0b0011_0000) >> 4) << 16;
        let start_sector_top = u16::from(value & 0b0000_0011) << 8;
        (
            load_address_top,
            execution_address_top,
            length_top,
            start_sector_top,
        )
    }

    // Calculate the value of Byte 7 of the catalogue entry on Sector 2 of disc
    // https://beebwiki.mdfs.net/Acorn_DFS_disc_format
    fn make_extra_bits(
        load_address: u32,
        execution_address: u32,
        length: u32,
        start_sector: u16,
    ) -> Result<u8> {
        Ok(u8::try_from(
            ((load_address >> 16) << 2)
                + ((execution_address >> 16) << 6)
                + ((length >> 16) << 4)
                + (u32::from(start_sector) >> 8),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use crate::catalogue_entry::CatalogueEntry;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(0x03_0000, 0x03_0000, 0x00_0000, 0x0000, 0b1100_1100)]
    #[case(0x01_0000, 0x02_0000, 0x03_0000, 0x0200, 0b1011_0110)]
    fn extract_extra_bits(
        #[case] expected_load_address_top: u32,
        #[case] expected_execution_address_top: u32,
        #[case] expected_length_top: u32,
        #[case] expected_start_sector_top: u16,
        #[case] extra_bits: u8,
    ) {
        assert_eq!(
            (
                expected_load_address_top,
                expected_execution_address_top,
                expected_length_top,
                expected_start_sector_top
            ),
            CatalogueEntry::extract_extra_bits(extra_bits)
        );
    }

    #[rstest]
    #[case(0b1100_1100, 0x03_ffff, 0x03_ffff, 0x00_000e, 0x0002)]
    #[case(0b1011_0110, 0x01_0000, 0x02_0000, 0x03_0000, 0x0200)]
    fn make_extra_bits(
        #[case] expected_extra_bits: u8,
        #[case] load_address: u32,
        #[case] execution_address: u32,
        #[case] length: u32,
        #[case] start_sector: u16,
    ) -> Result<()> {
        assert_eq!(
            expected_extra_bits,
            CatalogueEntry::make_extra_bits(load_address, execution_address, length, start_sector)?
        );
        Ok(())
    }
}
