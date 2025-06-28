use anyhow::{Result, bail};
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

pub fn open_for_write(path: &Path, overwrite: bool) -> Result<File> {
    let result = if overwrite {
        File::create(path)
    } else {
        File::create_new(path)
    };

    match result {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            bail!(
                "output file {path} already exists: pass --overwrite to overwrite",
                path = path.display()
            )
        }
        Err(e) => bail!(e),
    }
}

pub const fn is_disc_title_char(c: char) -> bool {
    c == '\0' || !c.is_ascii_control()
}

pub fn is_file_name_char(c: char) -> bool {
    const INVALID_CHARS: &str = ".:\"#* ";
    let value = c as u8;
    if (0x20..=0x7e).contains(&value) {
        !INVALID_CHARS.contains(c)
    } else {
        false
    }
}
