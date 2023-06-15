use crate::project_description_dto::ProjectDescriptionDto;
use crate::service::project_generator::ProjectGenerator;
use crate::hash;
use filepath::FilePath;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum DownloadServiceError {
    #[error("{0:?}")]
    Generating(#[from] io::Error),

    #[error("{0:?}")]
    Compressing(#[from] zip::result::ZipError),
}

pub fn generate(description_dto: &ProjectDescriptionDto) -> Result<Vec<u8>, Box<dyn Error>> {
    let project_hash = get_project_hash(description_dto);

    let generator = ProjectGenerator::new(&project_hash, description_dto);
    let root_dir = generator.generate_project()?;

    let zipped_project = zip_project(
        generator.get_hashed_dir().clone(),
        &description_dto.package_description.name,
        root_dir,
        zip::CompressionMethod::Deflated,
    )?;

    Ok(fs::read(zipped_project)?)
}

fn zip_project(
    hashed_dir: PathBuf,
    original_project_name: &str,
    root_dir: PathBuf,
    method: zip::CompressionMethod,
) -> Result<PathBuf, DownloadServiceError> {
    let it = WalkDir::new(&root_dir).into_iter();

    let zip_file = create_zip_file(hashed_dir, original_project_name)?;

    crate::service::compressor::zip_dir(
        &mut it.filter_map(|e| e.ok()),
        root_dir,
        &zip_file,
        method,
    )?;

    Ok(zip_file.path()?)
}

fn create_zip_file(
    hashed_dir: PathBuf,
    original_project_name: &str,
) -> Result<File, DownloadServiceError> {
    let mut path_to_compressed_project = hashed_dir;
    path_to_compressed_project.push(format!("{}{}", original_project_name, ".zip"));
    Ok(File::create(path_to_compressed_project)?)
}

fn get_project_hash(description_dto: &ProjectDescriptionDto) -> String {
    let current_time = chrono::Utc::now().to_string();

    let mut project_hash = hash!(format!("{}{}", current_time, hash!(description_dto))).to_string();

    tracing::info!("Project hash: {}", project_hash);

    while is_project_hash_exists(&project_hash) {
        project_hash = get_project_hash(description_dto);
    }

    project_hash
}

fn is_project_hash_exists(project_hash: &str) -> bool {
    let mut path =  PathBuf::from(dotenv::var("TEMP").unwrap());
    path.push(project_hash);
    path.exists()
}
