use crate::bbc_basic::{detokenize_source, is_bbc_basic_file};
use crate::constants::{INF_EXT, LOSSLESS_BBC_BASIC_EXT, LOSSY_BBC_BASIC_EXT, MANIFEST_VERSION};
use crate::dfs::{Catalogue, CatalogueEntry, FileSpec, SECTOR_BYTES, Side};
use crate::dsd_reader::DsdReader;
use crate::image_reader::ImageReader;
use crate::metadata::{FileType, KnownFileType, Manifest, make_inf_file};
use crate::path_util::add_extension;
use crate::ssd_reader::SsdReader;
use crate::util::open_for_write;
use anyhow::{Result, anyhow, bail};
use pathdiff::diff_paths;
use std::ffi::OsStr;
use std::fmt::Write as _;
use std::fs::{File, create_dir_all};
use std::io::{ErrorKind, Read, Write, copy};
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

pub fn run_extract(path: &Path, output_dir: &Path, opts: &ExtractOpts) -> Result<()> {
    if path.extension().and_then(OsStr::to_str) == Some("zip") {
        extract_from_zip(path, output_dir, opts)?;
    } else {
        extract_from_image(path, output_dir, opts)?;
    }
    Ok(())
}

// Zip file must contain exactly one .ssd or dsd file. All other files
// will be ignored.
fn extract_from_zip(path: &Path, output_dir: &Path, opts: &ExtractOpts) -> Result<()> {
    let mut zip_f = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            bail!("file {path} not found", path = path.display())
        }
        Err(e) => bail!(e),
    };

    let mut archive = ZipArchive::new(&mut zip_f)?;
    let mut image_files = Vec::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.is_file()
            && let Some(p) = file.enclosed_name()
        {
            let ext = p.extension().and_then(OsStr::to_str);
            if ext == Some("dsd") || ext == Some("ssd") {
                image_files.push((i, p));
            }
        }
    }

    let image_file_info = match image_files.len() {
        0 => bail!(
            "no disc images found in archive {path}",
            path = path.display()
        ),
        1 => image_files.first().unwrap(),
        _ => bail!(
            "more than one disc image was found in archive {path}",
            path = path.display()
        ),
    };

    let mut archive_file = archive.by_index(image_file_info.0)?;
    let mut f = tempfile()?;
    copy(&mut archive_file, &mut f)?;

    match image_file_info.1.extension().and_then(OsStr::to_str) {
        Some("dsd") => {
            let reader = DsdReader::new(f, SECTOR_BYTES)?;
            extract_all(path, output_dir, opts, reader)
        }
        Some("ssd") => {
            let reader = SsdReader::new(f, SECTOR_BYTES)?;
            extract_all(path, output_dir, opts, reader)
        }
        _ => bail!("unsupported file type {path}", path = path.display()),
    }
}

fn extract_from_image(path: &Path, output_dir: &Path, opts: &ExtractOpts) -> Result<()> {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            bail!("file {path} not found", path = path.display())
        }
        Err(e) => bail!(e),
    };

    match path.extension().and_then(OsStr::to_str) {
        Some("dsd") => {
            let reader = DsdReader::new(f, SECTOR_BYTES)?;
            extract_all(path, output_dir, opts, reader)
        }
        Some("ssd") => {
            let reader = SsdReader::new(f, SECTOR_BYTES)?;
            extract_all(path, output_dir, opts, reader)
        }
        _ => bail!("unsupported file type {path}", path = path.display()),
    }
}

fn extract_all<R: ImageReader>(
    path: &Path,
    output_dir: &Path,
    opts: &ExtractOpts,
    mut reader: R,
) -> Result<()> {
    let catalogues = Catalogue::from_image_reader(&mut reader)?;
    let double_sided = catalogues.len() > 1;
    for (i, catalogue) in catalogues.into_iter().enumerate() {
        let side = Side::try_from(i)?;

        let (output_dir, manifest_path) = if double_sided {
            let output_dir = output_dir.join(format!("side{i}"));
            let manifest_path = make_manifest_path(path, &output_dir, Some(side))?;
            (output_dir, manifest_path)
        } else {
            let manifest_path = make_manifest_path(path, output_dir, None)?;
            (output_dir.to_path_buf(), manifest_path)
        };

        if !output_dir.exists() {
            create_dir_all(&output_dir)?;
        }

        extract_single_side(
            side,
            catalogue,
            &manifest_path,
            &output_dir,
            opts,
            &mut reader,
        )?;
    }
    Ok(())
}

fn extract_single_side<R: ImageReader>(
    side: Side,
    catalogue: Catalogue,
    manifest_path: &Path,
    output_dir: &Path,
    opts: &ExtractOpts,
    reader: &mut R,
) -> Result<()> {
    let mut entries = catalogue.entries;
    entries.sort_by(|a, b| FileSpec::compare(&a.descriptor, &b.descriptor));

    let extracted_files = entries
        .iter()
        .map(|entry| {
            let file_type = extract_file(side, output_dir, opts, entry, reader)?;
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

    let manifest_file = open_for_write(manifest_path, opts.overwrite)?;
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

fn make_manifest_path(path: &Path, output_dir: &Path, side: Option<Side>) -> Result<PathBuf> {
    let mut file_name = String::new();
    file_name.push_str(
        path.file_stem()
            .and_then(OsStr::to_str)
            .ok_or_else(|| anyhow!("could not get file name from {path}", path = path.display()))?,
    );
    match side {
        Some(side) => write!(file_name, "-side{side}.json", side = u8::from(side))?,
        None => file_name.push_str(".json"),
    }
    Ok(output_dir.join(file_name))
}

fn extract_file<R: ImageReader>(
    side: Side,
    output_dir: &Path,
    opts: &ExtractOpts,
    entry: &CatalogueEntry,
    reader: &mut R,
) -> Result<(PathBuf, FileType)> {
    let d = &entry.descriptor;

    let mut bytes = vec![0; u32::from(entry.length) as usize];
    reader.read_bytes(side, entry.start_sector, &mut bytes)?;

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
