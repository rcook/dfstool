use crate::catalogue::Catalogue;
use anyhow::Result;
use std::path::Path;

pub fn do_show(ssd_path: &Path) -> Result<()> {
    let catalogue = Catalogue::from_file(ssd_path)?;
    println!("Title: \"{}\"", catalogue.disc_title);
    println!("Cycle number: {}", catalogue.cycle_number);
    println!("File number: {}", catalogue.file_offset.number());
    println!("Boot: {:?}", catalogue.boot_option);
    println!("Sectors: {:?}", u16::from(catalogue.disc_size));
    println!("Files:");
    for entry in &catalogue.entries {
        let d = &entry.descriptor;

        let extra = String::from(if d.locked { " (locked)" } else { "" });
        println!(
            "  {directory}.{file_name:<10} {load_address:06X} {execution_address:06X} {length:06X} {start_sector}{extra}",
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
