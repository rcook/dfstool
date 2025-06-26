use crate::constants::{SSD_CONTENT_FILE_EXT, SSD_METADATA_FILE_EXT};
use crate::file_descriptor::FileDescriptor;
use anyhow::{Result, anyhow, bail};
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::{File, read_dir};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Input {
    _content_path: PathBuf,
    metadata_path: PathBuf,
}

pub fn do_make(_ssd_path: &Path, _overwrite: bool) -> Result<()> {
    let input_dir = current_dir()?;
    let d = match read_dir(&input_dir) {
        Ok(d) => d,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            bail!("directory {dir} not found", dir = input_dir.display())
        }
        Err(e) => bail!(e),
    };

    let mut inputs = Vec::new();
    for entry in d {
        let entry = entry?;
        if entry.path().extension().and_then(OsStr::to_str) == Some(SSD_CONTENT_FILE_EXT) {
            let content_path = entry.path();
            let dir = content_path
                .parent()
                .ok_or_else(|| anyhow!("cannot get parent directory"))?;
            let stem = content_path
                .file_stem()
                .and_then(OsStr::to_str)
                .ok_or_else(|| anyhow!("cannot get file stem"))?;

            let metadata_path = dir.join(format!("{stem}.{ext}", ext = SSD_METADATA_FILE_EXT));
            if metadata_path.is_file() {
                inputs.push(Input {
                    _content_path: content_path,
                    metadata_path,
                });
            }
        }
    }

    for input in inputs {
        let f = File::open(input.metadata_path)?;
        let d = serde_json::from_reader::<_, FileDescriptor>(f)?;
        println!("{d:?}");
    }

    Ok(())
}
