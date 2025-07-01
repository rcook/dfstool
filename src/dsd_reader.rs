use crate::dfs::{SectorBytes, SectorSize, Side};
use crate::image_reader::ImageReader;
use anyhow::Result;
use std::io::{Read, Seek};

pub struct DsdReader<R: Read + Seek> {
    sector_bytes: SectorBytes,
    _reader: R,
}

impl<R: Read + Seek> DsdReader<R> {
    #[allow(unused)]
    pub const fn new(mut reader: R, sector_bytes: SectorBytes) -> Self {
        Self {
            sector_bytes,
            _reader: reader,
        }
    }
}

impl<R: Read + Seek> ImageReader for DsdReader<R> {
    fn sector_bytes(&self) -> SectorBytes {
        self.sector_bytes
    }

    fn read_bytes(
        &mut self,
        _side: Side,
        _start_sector: SectorSize,
        _buffer: &mut [u8],
    ) -> Result<()> {
        todo!()
    }
}
