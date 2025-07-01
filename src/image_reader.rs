use crate::dfs::{SectorBytes, SectorSize, Side};
use anyhow::Result;

#[allow(unused)]
pub trait ImageReader {
    fn sides(&self) -> u8;
    fn stream_len(&self) -> u64;
    fn sector_bytes(&self) -> SectorBytes;
    fn read_bytes(&mut self, side: Side, start_sector: SectorSize, buffer: &mut [u8])
    -> Result<()>;
}
