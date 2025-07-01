use crate::dfs::{SECTORS_PER_TRACK, SIDES_PER_DISC, SectorBytes, SectorSize, Side};
use crate::image_reader::ImageReader;
use anyhow::Result;
use std::io::{Read, Seek, SeekFrom};

pub struct DsdReader<R: Read + Seek> {
    stream_len: u64,
    sector_bytes: SectorBytes,
    reader: R,
}

impl<R: Read + Seek> DsdReader<R> {
    pub fn new(mut reader: R, sector_bytes: SectorBytes) -> Result<Self> {
        let stream_len = reader.seek(SeekFrom::End(0))?;
        Ok(Self {
            stream_len,
            sector_bytes,
            reader,
        })
    }

    fn read_single_sector(&mut self, side: Side, sector: usize, buffer: &mut [u8]) -> Result<()> {
        let sector_bytes = usize::from(self.sector_bytes);
        assert!(buffer.len() <= sector_bytes);

        let (track, r) = (sector / SECTORS_PER_TRACK, sector % SECTORS_PER_TRACK);
        let track_offset =
            sector_bytes * SECTORS_PER_TRACK * (track * SIDES_PER_DISC + usize::from(side));
        let sector_offset = track_offset + r * sector_bytes;

        self.reader
            .seek(SeekFrom::Start(u64::try_from(sector_offset)?))?;

        self.reader.read_exact(buffer)?;

        Ok(())
    }
}

impl<R: Read + Seek> ImageReader for DsdReader<R> {
    fn sides(&self) -> u8 {
        2
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
        let start_sector = usize::from(u16::from(start_sector));
        let sector_bytes = usize::from(self.sector_bytes);
        let (q, r) = (buffer.len() / sector_bytes, buffer.len() % sector_bytes);

        let mut ptr = 0;
        for sector in start_sector..start_sector + q {
            self.read_single_sector(side, sector, &mut buffer[ptr..ptr + sector_bytes])?;
            ptr += sector_bytes;
        }

        if r > 0 {
            self.read_single_sector(side, start_sector + q, &mut buffer[ptr..ptr + r])?;
        }

        Ok(())
    }
}
