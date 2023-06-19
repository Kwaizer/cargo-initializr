use crate::push;
use crate::service::compressor::CompressingError::StringConversion;
use filepath::FilePath;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;
use zip::CompressionMethod;

#[derive(Error, Debug)]
pub enum CompressingError {
    #[error("{0:?}")]
    Zipping(#[from] zip::result::ZipError),

    #[error("{0:?}")]
    IoError(#[from] io::Error),

    #[error("String conversion error with: {0:?}")]
    StringConversion(PathBuf),
}

pub fn zip_project(
    hashed_dir: PathBuf,
    original_project_name: &str,
    method: CompressionMethod,
) -> Result<PathBuf, CompressingError> {
    let it = WalkDir::new(&hashed_dir).into_iter();

    let zip_file = create_zip_file(&hashed_dir, original_project_name)?;

    zip_directory(
        &mut it.filter_map(|e| e.ok()),
        hashed_dir,
        &zip_file,
        method,
    )?;

    Ok(zip_file.path()?)
}

fn zip_directory<T>(
    iterator: &mut dyn Iterator<Item = DirEntry>,
    prefix: PathBuf,
    writer: T,
    method: CompressionMethod,
) -> Result<(), CompressingError>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in iterator {
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .ok_or_else(|| StringConversion(path.to_path_buf()))?
                .eq("zip")
        {
            continue;
        }
        let name = path.strip_prefix(Path::new(&prefix)).unwrap();

        if path.is_file() {
            zip.start_file(
                name.to_str()
                    .ok_or_else(|| StringConversion(name.to_path_buf()))?,
                options,
            )?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(
                name.to_str()
                    .ok_or_else(|| StringConversion(name.to_path_buf()))?,
                options,
            )?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn create_zip_file(
    hashed_dir: &Path,
    original_project_name: &str,
) -> Result<File, CompressingError> {
    let path_to_compressed_project =
        push!(hashed_dir, format!("{}{}", original_project_name, ".zip"));
    Ok(File::create(path_to_compressed_project)?)
}
