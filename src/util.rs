pub fn is_disc_title_char(c: char) -> bool {
    c == '\0' || !c.is_ascii_control()
}

pub fn is_file_name_char(c: char) -> bool {
    const INVALID_CHARS: &str = ".:\"#* ";
    let value = c as u8;
    if !(0x20..=0x7e).contains(&value) {
        false
    } else {
        !INVALID_CHARS.contains(c)
    }
}
