use crate::util::is_ascii_printable;

pub const fn is_disc_title_char(c: char) -> bool {
    c == '\0' || !c.is_ascii_control()
}

pub fn is_file_name_char(c: char) -> bool {
    const INVALID_CHARS: &str = ".:\"#* ";
    is_ascii_printable(c as u8) && !INVALID_CHARS.contains(c)
}
