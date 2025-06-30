use crate::catalogue_entry::CatalogueEntry;
use crate::dfs_path::DfsPath;
use crate::file_descriptor::FileDescriptor;
use crate::util::open_for_write;
use anyhow::{Result, bail};
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::iter::Peekable;
use std::path::Path;

const LOCKED: u8 = 0x08;
const NOT_LOCKED: u8 = 0x00;

// Acorn File Server date word - always zero for now
const DEFAULT_DATESTAMP: u16 = 0;

// Reference: https://www.geraldholdsworth.co.uk/documents/DiscImage.pdf
pub fn make_inf_file(inf_path: &Path, entry: &CatalogueEntry, overwrite: bool) -> Result<()> {
    let mut writer = InfWriter::new(inf_path, overwrite)?;
    let d = &entry.descriptor;

    writer.write_field(&format!("{dir}.{f}", dir = d.directory, f = d.file_name))?;

    writer.write_field(&format!(
        "{load_address:06X}",
        load_address = d.load_address
    ))?;

    writer.write_field(&format!(
        "{execution_address:06X}",
        execution_address = d.execution_address
    ))?;

    writer.write_field(&format!("{length:06X}", length = entry.length))?;

    writer.write_field(&format!(
        "{access:02X}",
        access = if d.locked { LOCKED } else { NOT_LOCKED }
    ))?;

    writer.write_field(&format!("{DEFAULT_DATESTAMP:04X}"))?;

    writer.write_line_end()?;

    Ok(())
}

struct InfWriter {
    file: File,
    field_count: usize,
}

impl InfWriter {
    fn new(path: &Path, overwrite: bool) -> Result<Self> {
        let file = open_for_write(path, overwrite)?;
        Ok(Self {
            file,
            field_count: 0,
        })
    }

    fn write_field<T: Display>(&mut self, value: &T) -> Result<()> {
        fn is_valid_str(s: &str) -> bool {
            s.chars()
                .all(|c| c.is_ascii() && !c.is_ascii_control() && c != '"')
        }

        let s = value.to_string();
        if !is_valid_str(&s) {
            bail!("cannot encode string {s}")
        }

        if self.field_count > 0 {
            write!(self.file, " ")?;
        }

        if s.contains(' ') {
            write!(self.file, "\"{s}\"")?;
        } else {
            write!(self.file, "{s}")?;
        }

        self.field_count += 1;

        Ok(())
    }

    fn write_line_end(&mut self) -> Result<()> {
        writeln!(self.file)?;
        self.field_count = 0;
        Ok(())
    }
}

pub fn read_inf_file(path: &Path) -> Result<FileDescriptor> {
    fn parse_locked(s: &str) -> Result<bool> {
        if s == "L" {
            Ok(true)
        } else {
            Ok(s.parse::<u8>()? == LOCKED)
        }
    }

    let fields = read_fields(path)?;
    if fields.len() < 6 {
        bail!(
            ".inf file {path} is missing required fields",
            path = path.display()
        );
    }

    let p = fields[0].parse::<DfsPath>()?;
    let load_address = u32::from_str_radix(&fields[1], 16)?.try_into()?;
    let execution_address = u32::from_str_radix(&fields[2], 16)?.try_into()?;
    let locked = parse_locked(&fields[4])?;

    Ok(FileDescriptor::new(
        p.file_name,
        p.directory,
        locked,
        load_address,
        execution_address,
    ))
}

fn read_fields(path: &Path) -> Result<Vec<String>> {
    fn read_quoted(i: &mut Peekable<impl Iterator<Item = char>>) -> String {
        assert_eq!('"', i.next().unwrap());
        let mut s = String::new();
        for c in i.by_ref() {
            if c == '"' {
                break;
            }
            s.push(c);
        }
        s
    }

    fn read_unquoted(i: &mut Peekable<impl Iterator<Item = char>>) -> String {
        let mut s = String::new();
        while let Some(c) = i.peek() {
            if c.is_whitespace() {
                break;
            }
            s.push(i.next().unwrap());
        }
        s
    }

    let s = read_to_string(path)?;
    let mut i = s.chars().peekable();
    let mut fields = Vec::new();
    while let Some(c) = i.peek() {
        match c {
            '"' => fields.push(read_quoted(&mut i)),
            c if c.is_whitespace() => _ = i.next().unwrap(),
            _ => fields.push(read_unquoted(&mut i)),
        }
    }

    Ok(fields)
}
