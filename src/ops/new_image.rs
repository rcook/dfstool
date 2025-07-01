use crate::dfs::{
    BootOption, Catalogue, CatalogueEntry, FileCount, FileDescriptor, FileSpec, Length,
    SECTOR_BYTES, START_SECTOR, SectorSize, get_file_sector_count,
};
use crate::metadata::{Manifest, read_inf_file};
use crate::path_util::strip_extension;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use path_absolutize::Absolutize;
use std::fs::{File, create_dir_all, metadata};
use std::io::{Read, Write};
use std::path::Path;

pub fn new_image_file(
    output_path: &Path,
    overwrite: bool,
    manifest_dir: &Path,
    mut manifest: Manifest,
) -> Result<()> {
    manifest.inf_files.sort();
    manifest.files.sort_by(FileSpec::compare);

    let mut bytes = vec![0u8; u16::from(manifest.disc_size) as usize * usize::from(SECTOR_BYTES)];

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

    if u16::from(start_sector) > u16::from(disc_size) {
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
    path: &Path,
    descriptor: FileDescriptor,
    start_sector: SectorSize,
) -> Result<(CatalogueEntry, SectorSize)> {
    let m = metadata(path)?;
    let length = Length::try_from(u32::try_from(m.len())?)?;
    let mut f = File::open(path)?;
    let start_offset = usize::from(start_sector) * usize::from(SECTOR_BYTES);
    let end_offset = start_offset + usize::try_from(m.len())?;
    f.read_exact(&mut bytes[start_offset..end_offset])?;

    Ok((
        CatalogueEntry::new(descriptor, length, start_sector),
        get_file_sector_count(length)?,
    ))
}
