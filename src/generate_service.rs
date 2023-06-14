use crate::generate_service::generator::ProjectGenerator;
use crate::generate_service::DownloadServiceError::{Compressing, Generating, Hashing};
use crate::project_description_dto::ProjectDescriptionDto;
use crate::{hash, throw};
use filepath::FilePath;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;
use walkdir::WalkDir;

pub mod generator;

#[derive(Error, Debug)]
pub enum DownloadServiceError {
    #[error("{0:?}")]
    Hashing(String),

    #[error("{0:?}")]
    Generating(String),

    #[error("{0:?}")]
    Compressing(String),
}

pub fn generate(description_dto: &ProjectDescriptionDto) -> Result<Vec<u8>, Box<dyn Error>> {
    let project_hash = get_project_hash(description_dto);
    if let Err(e) = project_hash {
        throw!(Hashing(e.to_string()));
    };

    let generator = ProjectGenerator::new(&project_hash.unwrap(), description_dto);
    let root_dir = generator.generate_project();
    if let Err(e) = root_dir {
        throw!(Generating(e.to_string()));
    };

    let zipped_project = zip_project(
        generator.get_hashed_dir().clone(),
        &description_dto.package_description.name,
        root_dir.unwrap(),
        zip::CompressionMethod::Deflated,
    );

    if let Err(e) = zipped_project {
        throw!(Compressing(e.to_string()));
    }

    Ok(fs::read(zipped_project.unwrap())?)
}

fn zip_project(
    hashed_dir: PathBuf,
    original_project_name: &str,
    root_dir: PathBuf,
    method: zip::CompressionMethod,
) -> Result<PathBuf, Box<dyn Error>> {
    let it = WalkDir::new(&root_dir).into_iter();
    let root_dir_path = root_dir.to_str().ok_or_else(|| {
        tracing::error!("Could not get root directory path as &str");
        Compressing("Could not get root directory path as &str".to_string())
    })?;

    let zip_file = create_zip_file(hashed_dir, original_project_name)?;

    crate::compressor::zip_dir(
        &mut it.filter_map(|e| e.ok()),
        root_dir_path,
        &zip_file,
        method,
    )?;

    Ok(zip_file.path()?)
}

fn create_zip_file(
    hashed_dir: PathBuf,
    original_project_name: &str,
) -> Result<File, Box<dyn Error>> {
    let mut path_to_compressed_project = hashed_dir;
    path_to_compressed_project.push(format!("{}{}", original_project_name, ".zip"));
    Ok(File::create(path_to_compressed_project)?)
}

fn get_project_hash(description_dto: &ProjectDescriptionDto) -> Result<String, Box<dyn Error>> {
    let current_time = chrono::Utc::now().to_string();

    let mut project_hash = hash!(format!("{}{}", current_time, hash!(description_dto))).to_string();

    tracing::info!("Project hash: {}", project_hash);

    while is_project_hash_exists(&project_hash)? {
        project_hash = get_project_hash(description_dto)?;
    }

    Ok(project_hash)
}

fn is_project_hash_exists(project_hash: &str) -> Result<bool, Box<dyn Error>> {
    let path = format!("{}{}{}", dotenv::var("TEMP").unwrap(), "/", project_hash);
    Ok(PathBuf::from_str(&path)?.exists())
}
