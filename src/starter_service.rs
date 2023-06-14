use crate::starter_dto::StarterDto;
use crate::starter_service::StarterServiceError::{
    InvalidStarterManifest, MissingPackageSection, NoStartersProvided,
};
use std::path::PathBuf;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum StarterServiceError {
    #[error("Could read starter manifest: {0:?}")]
    InvalidStarterManifest(PathBuf),

    #[error("No starters provided")]
    NoStartersProvided,

    #[error("Cannot read Metadata of starter file")]
    CannotReadMetadataOfStarterFile(#[from] walkdir::Error),

    #[error("Missing package section in: {0:?} starter")]
    MissingPackageSection(PathBuf),
}

pub fn get_starters() -> Result<Vec<StarterDto>, StarterServiceError> {
    let starters_dir = dotenv::var("CONTENT").unwrap();
    let mut starters = Vec::new();

    for starter_file in WalkDir::new(starters_dir)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if starter_file.metadata()?.is_dir() {
            continue;
        }

        let stater_path = starter_file.path().to_path_buf();

        let starter_dto = match read_starter(stater_path) {
            Ok(dto) => dto,
            Err(e) => {
                tracing::error!("{e:?}");
                continue;
            }
        };

        starters.push(starter_dto)
    }

    if starters.is_empty() {
        return Err(NoStartersProvided);
    }

    tracing::debug!("Starters: {:?}", starters);
    Ok(starters)
}

fn read_starter(stater_path: PathBuf) -> Result<StarterDto, StarterServiceError> {
    let starter = cargo_toml::Manifest::from_path(&stater_path)
        .map_err(|_| InvalidStarterManifest(stater_path.clone()))?;
    let package_section = starter.package.ok_or(MissingPackageSection(stater_path))?;
    let starter_name = package_section.name().to_string();
    let starter_description = package_section
        .description()
        .unwrap_or("Missing description")
        .to_string();
    let dependencies = starter.dependencies.into_keys().collect();

    let starter_dto = StarterDto::new(starter_name, dependencies, starter_description);
    Ok(starter_dto)
}
