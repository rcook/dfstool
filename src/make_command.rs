use crate::boot_option::BootOption;
use crate::catalogue::Catalogue;
use crate::catalogue_entry::CatalogueEntry;
use crate::constants::{SECTOR_SIZE, SSD_CONTENT_FILE_EXT, SSD_METADATA_FILE_EXT, START_SECTOR};
use crate::cycle_number::CycleNumber;
use crate::file_count::FileCount;
use crate::file_descriptor::FileDescriptor;
use crate::u10::DiscSize;
use crate::u18::Length;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use std::cmp::Ordering;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::{File, metadata, read_dir};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

pub fn do_make(ssd_path: &Path, overwrite: bool) -> Result<()> {
    struct Input {
        content_path: PathBuf,
        metadata_path: PathBuf,
    }

    struct FileInfo {
        descriptor: FileDescriptor,
        content_path: PathBuf,
    }

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
            Ok(FileInfo {
                descriptor: serde_json::from_reader::<_, FileDescriptor>(f)?,
                content_path: input.content_path,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    file_infos.sort_by(|a, b| {
        let a = &a.descriptor;
        let b = &b.descriptor;
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

    let disc_size: DiscSize = 800.try_into()?;
    let mut bytes = vec![0u8; disc_size.as_usize() * SECTOR_SIZE];

    let mut start_sector = START_SECTOR;
    let mut entries = Vec::new();
    for file_info in file_infos {
        let m = metadata(&file_info.content_path)?;
        let length: Length = <u32 as TryFrom<u64>>::try_from(m.len())?.try_into()?;
        let temp_start_sector = <u16 as TryFrom<usize>>::try_from(start_sector)?.try_into()?;
        entries.push(CatalogueEntry::new(
            file_info.descriptor,
            length,
            temp_start_sector,
        ));

        let temp_len = m.len() as usize;
        let (q, r) = (temp_len / SECTOR_SIZE, temp_len % SECTOR_SIZE);
        let sector_count = q + if r > 0 { 1 } else { 0 };

        let mut f = File::open(&file_info.content_path)?;
        let start_offset = start_sector * SECTOR_SIZE;
        let end_offset = start_offset + temp_len;
        f.read_exact(&mut bytes[start_offset..end_offset])?;

        start_sector += sector_count;
    }

    if start_sector > disc_size.as_usize() {
        bail!("exceeded capacity of disc")
    }

    let disc_title = "DISC".parse()?;
    let cycle_number = CycleNumber::new(95)?;
    let file_count: FileCount = entries.len().try_into()?;
    let file_offset = file_count.into();
    let boot_option = BootOption::Exec;

    let catalogue = Catalogue::new(
        disc_title,
        cycle_number,
        file_offset,
        boot_option,
        disc_size,
        entries,
    );

    catalogue.write_to(&mut bytes)?;

    let mut f = open_for_write(ssd_path, overwrite)?;
    f.write_all(&bytes)?;

    Ok(())
}
