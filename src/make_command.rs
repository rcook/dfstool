use crate::boot_option::BootOption;
use crate::catalogue::Catalogue;
use crate::catalogue_entry::CatalogueEntry;
use crate::constants::{MANIFEST_VERSION, SECTOR_SIZE, START_SECTOR};
use crate::file_count::FileCount;
use crate::file_descriptor::FileDescriptor;
use crate::file_spec::FileSpec;
use crate::inf::read_inf_file;
use crate::length::Length;
use crate::manifest::Manifest;
use crate::path_util::strip_extension;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use path_absolutize::Absolutize;
use std::fs::{File, create_dir_all, metadata};
use std::io::{Read, Write};
use std::path::Path;

pub fn do_make(manifest_path: &Path, output_path: &Path, overwrite: bool) -> Result<()> {
    let manifest_dir = manifest_path.parent().ok_or_else(|| {
        anyhow!(
            "cannot get parent directory from {manifest_path}",
            manifest_path = manifest_path.display()
        )
    })?;

    let manifest_file = File::open(manifest_path)?;
    let mut manifest = serde_json::from_reader::<_, Manifest>(manifest_file)?;
    if let Some(version) = manifest.version
        && version != MANIFEST_VERSION
    {
        bail!("unsupported manifest version {version}");
    }

    manifest.inf_files.sort();
    manifest.files.sort_by(FileSpec::compare);

    let mut bytes = vec![0u8; u16::from(manifest.disc_size) as usize * SECTOR_SIZE];

    let mut start_sector = START_SECTOR;
    let mut entries = Vec::new();

    for inf_file in manifest.inf_files {
        let p = inf_file.absolutize_from(manifest_dir)?;
        let descriptor = read_inf_file(&p)?;
        let content_path = strip_extension(&p)?;
        let (entry, sector_count) =
            write_content(&mut bytes, &content_path, descriptor, start_sector)?;
        entries.push(entry);
        start_sector += sector_count;
    }

    for file in manifest.files {
        let content_path = file.content_path.absolutize_from(manifest_dir)?;
        let (entry, sector_count) = write_content(
            &mut bytes,
            &content_path,
            file.to_file_descriptor(),
            start_sector,
        )?;
        entries.push(entry);
        start_sector += sector_count;
    }

    let disc_size = manifest.disc_size;

    if start_sector > u16::from(disc_size) as usize {
        bail!("exceeded capacity of disc")
    }

    let file_count: FileCount = u8::try_from(entries.len())?.try_into()?;
    let file_offset = file_count.into();
    let boot_option = BootOption::Exec;

    let catalogue = Catalogue::new(
        manifest.disc_title.unwrap_or_else(|| "".parse().unwrap()),
        manifest.cycle_number,
        file_offset,
        boot_option,
        disc_size,
        entries,
    );

    catalogue.write_to_catalogue(&mut bytes)?;

    let output_dir = output_path
        .parent()
        .ok_or_else(|| anyhow!("cannot get parent"))?;
    create_dir_all(output_dir)?;

    let mut output_file = open_for_write(output_path, overwrite)?;
    output_file.write_all(&bytes)?;

    Ok(())
}

fn write_content(
    bytes: &mut [u8],
    content_path: &Path,
    descriptor: FileDescriptor,
    start_sector: usize,
) -> Result<(CatalogueEntry, usize)> {
    let m = metadata(content_path)?;
    let length: Length = <u32 as TryFrom<u64>>::try_from(m.len())?.try_into()?;
    let temp_start_sector = <u16 as TryFrom<usize>>::try_from(start_sector)?.try_into()?;
    let temp_len = usize::try_from(m.len())?;
    let (q, r) = (temp_len / SECTOR_SIZE, temp_len % SECTOR_SIZE);
    let sector_count = q + usize::from(r > 0);
    let mut f = File::open(content_path)?;
    let start_offset = start_sector * SECTOR_SIZE;
    let end_offset = start_offset + temp_len;
    f.read_exact(&mut bytes[start_offset..end_offset])?;
    Ok((
        CatalogueEntry::new(descriptor, length, temp_start_sector),
        sector_count,
    ))
}
