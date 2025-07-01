use crate::dfs::SECTOR_SIZE;
use crate::util::is_ascii_printable;

pub const fn is_disc_title_char(c: char) -> bool {
    c == '\0' || !c.is_ascii_control()
}

pub fn is_file_name_char(c: char) -> bool {
    const INVALID_CHARS: &str = ".:\"#* ";
    is_ascii_printable(c as u8) && !INVALID_CHARS.contains(c)
}

pub fn get_file_sector_count(length: usize) -> usize {
    let (q, r) = (length / SECTOR_SIZE, length % SECTOR_SIZE);
    q + usize::from(r > 0)
}
