use crate::dfs::{SectorBytes, SectorSize, Side};
use crate::image_reader::ImageReader;
use anyhow::{Result, bail};
use std::io::{Read, Seek, SeekFrom};

pub struct SsdReader<R: Read + Seek> {
    stream_len: u64,
    sector_bytes: SectorBytes,
    reader: R,
}

impl<R: Read + Seek> SsdReader<R> {
    pub fn new(mut reader: R, sector_bytes: SectorBytes) -> Result<Self> {
        let stream_len = reader.seek(SeekFrom::End(0))?;
        Ok(Self {
            stream_len,
            sector_bytes,
            reader,
        })
    }
}

impl<R: Read + Seek> ImageReader for SsdReader<R> {
    fn sides(&self) -> u8 {
        1
    }

    fn stream_len(&self) -> u64 {
        self.stream_len
    }

    fn sector_bytes(&self) -> SectorBytes {
        self.sector_bytes
    }

    fn read_bytes(
        &mut self,
        side: Side,
        start_sector: SectorSize,
        buffer: &mut [u8],
    ) -> Result<()> {
        if u8::from(side) > 0 {
            bail!("SSD reader supports one side only")
        }

        self.reader.seek(SeekFrom::Start(
            u64::from(start_sector) * u64::from(self.sector_bytes),
        ))?;

        self.reader.read_exact(buffer)?;

        Ok(())
    }
}
