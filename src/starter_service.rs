use crate::starter_dto::StarterDto;
use crate::starter_service::StarterServiceError::InvalidStarterManifest;
use std::error::Error;
use std::path::PathBuf;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum StarterServiceError {
    #[error("Could read starter manifest: {0:?}")]
    InvalidStarterManifest(PathBuf),
}

pub fn get_starters() -> Result<Vec<StarterDto>, Box<dyn Error>> {
    let starters_dir = dotenv::var("CONTENT")?;
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
                tracing::error!(e);
                continue;
            }
        };

        starters.push(starter_dto)
    }

    tracing::debug!("Starters: {:?}", starters);
    Ok(starters)
}

fn read_starter(stater_path: PathBuf) -> Result<StarterDto, Box<dyn Error>> {
    let starter = cargo_toml::Manifest::from_path(&stater_path)?;
    let starter_package = starter.package.ok_or(InvalidStarterManifest(stater_path))?;
    let starter_name = starter_package.name().to_string();
    let starter_description = starter_package
        .description()
        .unwrap_or("Missing description")
        .to_string();
    let crates = starter.dependencies.into_keys().collect();

    let starter_dto = StarterDto::new(starter_name, crates, starter_description);
    Ok(starter_dto)
}
