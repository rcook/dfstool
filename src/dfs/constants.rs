use crate::dfs::{SectorBytes, SectorSize};

pub const START_SECTOR: SectorSize = SectorSize::TWO;

pub const SIDES_PER_DISC: usize = 2;

pub const SECTORS_PER_TRACK: usize = 10;

pub const SECTOR_BYTES: SectorBytes = 256;

pub const DFS_TOTAL_FILES: u8 = 31;
