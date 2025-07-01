use crate::dfs::{SectorBytes, SectorSize, Side};
use anyhow::Result;

#[allow(unused)]
pub trait ImageReader {
    fn sector_bytes(&self) -> SectorBytes;
    fn read_bytes(&mut self, side: Side, start_sector: SectorSize, buffer: &mut [u8])
    -> Result<()>;
}
