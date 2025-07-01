use crate::dfs::{
    Catalogue, FileSpec, SECTOR_BYTES, START_SECTOR, SectorSize, get_file_sector_count,
};
use anyhow::{Error, Result};
use std::path::Path;

pub fn run_show(ssd_path: &Path) -> Result<()> {
    let catalogue = Catalogue::from_file(ssd_path)?;
    println!(
        "{label:<13}: {value}",
        label = "Title",
        value = catalogue.disc_title
    );
    println!(
        "{label:<13}: {value}",
        label = "Cycle number",
        value = catalogue.cycle_number
    );
    println!(
        "{label:<13}: {value}",
        label = "File count",
        value = catalogue.file_offset.number()
    );
    println!(
        "{label:<13}: {value:?}",
        label = "Boot option",
        value = catalogue.boot_option
    );

    let total_sectors = usize::from(u16::from(catalogue.disc_size));

    println!(
        "{label:<13}: {value}",
        label = "Total sectors",
        value = total_sectors
    );

    let used_sectors = usize::from(
        catalogue
            .entries
            .iter()
            .try_fold(SectorSize::ZERO, |acc, entry| {
                Ok::<SectorSize, Error>(acc + get_file_sector_count(entry.length)?)
            })?,
    );
    let free_sectors = total_sectors - used_sectors - usize::from(START_SECTOR);
    let free_bytes = free_sectors * usize::from(SECTOR_BYTES);

    println!(
        "{label:<13}: {value} ({free_bytes} bytes)",
        label = "Free sectors",
        value = free_sectors
    );

    println!("Files:");

    let mut entries = catalogue.entries;
    entries.sort_by(|a, b| FileSpec::compare(&a.descriptor, &b.descriptor));

    for entry in &entries {
        let d = &entry.descriptor;

        let extra = String::from(if d.locked { " L" } else { "" });
        println!(
            "  {directory}.{file_name:<7} {load_address:06X} {execution_address:06X} {length:06X} {start_sector:03X}{extra}",
            directory = d.directory,
            file_name = d.file_name.to_string(),
            load_address = d.load_address,
            execution_address = d.execution_address,
            length = entry.length,
            start_sector = entry.start_sector,
            extra = extra
        );
    }

    Ok(())
}
