use crate::dfs::{SectorBytes, SectorSize, Side};
use crate::image_reader::ImageReader;
use anyhow::{Result, bail};
use std::io::{Read, Seek, SeekFrom};

#[allow(unused)]
pub struct SsdReader<R: Read + Seek> {
    sector_bytes: SectorBytes,
    len: u64,
    reader: R,
}

impl<R: Read + Seek> SsdReader<R> {
    #[allow(unused)]
    pub fn new(mut reader: R, sector_bytes: SectorBytes) -> Result<Self> {
        let len = reader.seek(SeekFrom::End(0))?;
        Ok(Self {
            sector_bytes,
            len,
            reader,
        })
    }
}

impl<R: Read + Seek> ImageReader for SsdReader<R> {
    fn sector_bytes(&self) -> SectorBytes {
        self.sector_bytes
    }

    fn read_bytes(
        &mut self,
        side: Side,
        start_sector: SectorSize,
        buffer: &mut [u8],
    ) -> Result<()> {
        if side != 0 {
            bail!("SSD reader supports one side only")
        }

        self.reader.seek(SeekFrom::Start(
            u64::from(start_sector) * u64::from(self.sector_bytes),
        ))?;

        self.reader.read_exact(buffer)?;

        Ok(())
    }
}
