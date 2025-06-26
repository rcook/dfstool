use crate::catalogue::Catalogue;
use anyhow::Result;
use std::fs::File;
use std::path::Path;

pub fn do_show(ssd_path: &Path) -> Result<()> {
    let mut ssd_file = File::open(ssd_path)?;
    let catalogue = Catalogue::read(&mut ssd_file)?;
    catalogue.show();
    Ok(())
}
