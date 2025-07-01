use crate::bbc_basic::{detokenize_source, is_bbc_basic_file};
use crate::constants::{INF_EXT, LOSSLESS_BBC_BASIC_EXT, LOSSY_BBC_BASIC_EXT, MANIFEST_VERSION};
use crate::dfs::{Catalogue, CatalogueEntry, FileSpec};
use crate::metadata::{FileType, KnownFileType, Manifest, make_inf_file};
use crate::path_util::add_extension;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use pathdiff::diff_paths;
use std::ffi::OsStr;
use std::fs::{File, create_dir_all};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write, copy};
use std::path::{Path, PathBuf};
use tempfile::tempfile;
use zip::ZipArchive;

#[allow(clippy::struct_excessive_bools)]
pub struct ExtractOpts {
    pub overwrite: bool,
    pub detokenize: bool,
    pub lossless: bool,
    pub inf: bool,
}

pub fn do_extract(input_path: &Path, output_dir: &Path, opts: &ExtractOpts) -> Result<()> {
    if input_path.extension().and_then(OsStr::to_str) == Some("zip") {
        extract_from_zip(input_path, output_dir, opts)?;
    } else {
        extract_from_ssd(input_path, output_dir, opts)?;
    }
    Ok(())
}

// Zip file must contain exactly one .ssd file. All other files
// will be ignored.
fn extract_from_zip(input_path: &Path, output_dir: &Path, opts: &ExtractOpts) -> Result<()> {
    let mut zip_file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => bail!(
            "input file {input_path} not found",
            input_path = input_path.display()
        ),
        Err(e) => bail!(e),
    };

    let mut archive = ZipArchive::new(&mut zip_file)?;
    let mut ssd_files = Vec::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.is_file()
            && let Some(p) = file.enclosed_name()
            && p.extension().and_then(OsStr::to_str) == Some("ssd")
        {
            ssd_files.push((i, p));
        }
    }

    let ssd_file_info = match ssd_files.len() {
        0 => bail!(
            "no .ssd files found in archive {input_path}",
            input_path = input_path.display()
        ),
        1 => ssd_files.first().unwrap(),
        _ => bail!(
            "more than one .ssd file was found in archive {input_path}",
            input_path = input_path.display()
        ),
    };

    let mut archive_file = archive.by_index(ssd_file_info.0)?;
    let mut input_file = tempfile()?;
    copy(&mut archive_file, &mut input_file)?;
    input_file.rewind()?;

    extract_files(input_path, output_dir, opts, input_file)
}

fn extract_from_ssd(input_path: &Path, output_dir: &Path, opts: &ExtractOpts) -> Result<()> {
    let input_file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => bail!(
            "input file {input_path} not found",
            input_path = input_path.display()
        ),
        Err(e) => bail!(e),
    };

    extract_files(input_path, output_dir, opts, input_file)
}

fn extract_files<R: Read + Seek>(
    input_path: &Path,
    output_dir: &Path,
    opts: &ExtractOpts,
    mut input_file: R,
) -> Result<()> {
    if !output_dir.exists() {
        create_dir_all(output_dir)?;
    }

    let catalogue = Catalogue::from_reader(&mut input_file)?;

    let mut manifest_file_name = String::new();
    manifest_file_name.push_str(input_path.file_stem().and_then(OsStr::to_str).ok_or_else(
        || {
            anyhow!(
                "could not get file name from {input_path}",
                input_path = input_path.display()
            )
        },
    )?);
    manifest_file_name.push_str(".json");
    let manifest_path = output_dir.join(manifest_file_name);

    let mut entries = catalogue.entries;
    entries.sort_by(|a, b| FileSpec::compare(&a.descriptor, &b.descriptor));

    let extracted_files = entries
        .iter()
        .map(|entry| {
            let file_type = extract_file(output_dir, opts, entry, &mut input_file)?;
            Ok((entry, file_type))
        })
        .collect::<Result<Vec<_>>>()?;

    let (inf_files, files) = if opts.inf {
        let inf_files = extracted_files
            .into_iter()
            .map(|(entry, (content_path, _))| {
                let inf_path = add_extension(&content_path, INF_EXT)?;
                make_inf_file(&inf_path, entry, opts.overwrite)?;
                let rel_inf_path = diff_paths(inf_path, output_dir)
                    .ok_or_else(|| anyhow!("could not determine relative path"))?;
                Ok(rel_inf_path)
            })
            .collect::<Result<Vec<_>>>()?;
        (inf_files, Vec::new())
    } else {
        let files = extracted_files
            .into_iter()
            .map(|(entry, (_, file_type))| entry.descriptor.to_manifest_file(file_type))
            .collect();
        (Vec::new(), files)
    };

    let manifest_file = open_for_write(&manifest_path, opts.overwrite)?;
    serde_json::to_writer_pretty(
        manifest_file,
        &Manifest {
            version: Some(MANIFEST_VERSION),
            disc_title: Some(catalogue.disc_title),
            disc_size: catalogue.disc_size,
            boot_option: catalogue.boot_option,
            cycle_number: catalogue.cycle_number,
            inf_files,
            files,
        },
    )?;

    Ok(())
}

fn extract_file<R: Read + Seek>(
    output_dir: &Path,
    opts: &ExtractOpts,
    entry: &CatalogueEntry,
    mut input_file: R,
) -> Result<(PathBuf, FileType)> {
    let d = &entry.descriptor;
    let mut bytes = vec![0; u32::from(entry.length) as usize];
    input_file.seek(SeekFrom::Start(
        u64::from(u16::from(entry.start_sector)) * 256,
    ))?;
    input_file.read_exact(&mut bytes)?;
    let content_path = output_dir.join(d.content_path());
    let mut content_file = open_for_write(&content_path, opts.overwrite)?;
    content_file.write_all(&bytes)?;
    let is_bbc_basic = is_bbc_basic_file(&content_path)?;
    if opts.detokenize && is_bbc_basic {
        // Attempt to detokenize the file just in case it contains BASIC
        // Don't fail if it can't be detokenized
        _ = detokenize_file(&content_path, opts.overwrite, opts.lossless);
    }

    let file_type = FileType::Known(if is_bbc_basic {
        KnownFileType::BbcBasic
    } else {
        KnownFileType::Other
    });
    Ok((content_path, file_type))
}

fn detokenize_file(input_path: &Path, overwrite: bool, lossless: bool) -> Result<()> {
    let output_path = add_extension(
        input_path,
        if lossless {
            LOSSLESS_BBC_BASIC_EXT
        } else {
            LOSSY_BBC_BASIC_EXT
        },
    )?;

    let output_file = open_for_write(&output_path, overwrite)?;
    let mut input_file = File::open(input_path)?;
    let mut bytes = Vec::new();
    input_file.read_to_end(&mut bytes)?;
    detokenize_source(output_file, &bytes, lossless)
}
