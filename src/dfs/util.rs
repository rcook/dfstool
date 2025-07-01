use anyhow::Result;

use crate::dfs::{Length, SECTOR_BYTES, SectorSize};
use crate::util::is_ascii_printable;

pub const fn is_disc_title_char(c: char) -> bool {
    c == '\0' || !c.is_ascii_control()
}

pub fn is_file_name_char(c: char) -> bool {
    const INVALID_CHARS: &str = ".:\"#* ";
    is_ascii_printable(c as u8) && !INVALID_CHARS.contains(c)
}

pub fn get_file_sector_count(length: Length) -> Result<SectorSize> {
    let (q, r) = (
        u32::from(length) / u32::from(SECTOR_BYTES),
        u32::from(length) % u32::from(SECTOR_BYTES),
    );
    u16::try_from(q + u32::from(r > 0))?.try_into()
}
