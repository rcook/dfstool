use crate::catalogue::Catalogue;
use anyhow::Result;
use std::path::Path;

pub fn do_show(ssd_path: &Path) -> Result<()> {
    let catalogue = Catalogue::from_file(ssd_path)?;
    catalogue.show();
    Ok(())
}
