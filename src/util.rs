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

pub fn is_ascii_printable(value: u8) -> bool {
    (32..=126).contains(&value)
}
