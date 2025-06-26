use crate::boot_option::BootOption;
use crate::catalogue::Catalogue;
use crate::catalogue_entry::CatalogueEntry;
use crate::constants::{SECTOR_SIZE, SSD_CONTENT_FILE_EXT, SSD_METADATA_FILE_EXT, START_SECTOR};
use crate::file_count::FileCount;
use crate::file_descriptor::FileDescriptor;
use crate::u10::DiscSize;
use crate::u18::Length;
use anyhow::{Result, anyhow, bail};
use std::cmp::Ordering;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::{File, metadata, read_dir};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Input {
    content_path: PathBuf,
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
                    content_path,
                    metadata_path,
                });
            }
        }
    }

    let mut file_infos = inputs
        .into_iter()
        .map(|input| {
            let f = File::open(input.metadata_path)?;
            Ok((
                serde_json::from_reader::<_, FileDescriptor>(f)?,
                input.content_path,
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    file_infos.sort_by(|(a, _), (b, _)| {
        match a.directory.partial_cmp(&b.directory) {
            Some(ordering) if ordering != Ordering::Equal => return ordering,
            _ => {}
        }
        match a.file_name.partial_cmp(&b.file_name) {
            Some(ordering) if ordering != Ordering::Equal => return ordering,
            _ => {}
        }
        Ordering::Equal
    });

    let mut start_sector = START_SECTOR;
    let mut entries = Vec::new();
    for file_info in file_infos {
        let m = metadata(file_info.1)?;
        let length: Length = <u32 as TryFrom<u64>>::try_from(m.len())?.try_into()?;
        let temp_start_sector = <u16 as TryFrom<usize>>::try_from(start_sector)?.try_into()?;
        entries.push(CatalogueEntry::new(file_info.0, length, temp_start_sector));
        let temp_len = length.as_usize();
        let (q, r) = (temp_len / SECTOR_SIZE, temp_len % SECTOR_SIZE);
        let sectors = q + if r > 0 { 1 } else { 0 };
        start_sector += sectors;
    }

    let disc_title = "DISC".parse()?;
    let cycle_number = 0.try_into()?;
    let file_count: FileCount = entries.len().try_into()?;
    let file_offset = file_count.into();
    let boot_option = BootOption::None;
    let disc_size: DiscSize = 800.try_into()?;
    let _bytes = vec![0; disc_size.as_usize() * SECTOR_SIZE];
    let catalogue = Catalogue::new(
        disc_title,
        cycle_number,
        file_offset,
        boot_option,
        disc_size,
        entries,
    );

    catalogue.show();

    Ok(())
}
