use std::fs;
use std::path::PathBuf;

use common::starter::raw_starter::RawStarter;
use common::starter::starter_dto::StarterDto;
use common::starter::Starter;
use thiserror::Error;
use walkdir::WalkDir;

use crate::downcast;
use crate::service::starter_service::StarterServiceError::{
    CannotReadStarterFile,
    InvalidStarterManifest,
    MissingPackageSection,
    NoStartersProvided,
};
use crate::service::traits::StarterRepository;

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

    #[error("Cannot read: {0:?} starter")]
    CannotReadStarterFile(PathBuf),
}

#[derive(Clone)]
pub struct StarterService {
    starter_storage: Box<dyn StarterRepository>,
}

impl StarterService {
    pub async fn new(starter_storage: impl StarterRepository + 'static) -> Self {
        let mut starter_storage = starter_storage;
        starter_storage
            .load_starters()
            .await
            .expect("Cannot initialize starter repository.");
        Self {
            starter_storage: Box::new(starter_storage),
        }
    }

    pub async fn get_starters(&self) -> Result<Vec<Starter>, StarterServiceError> {
        self.starter_storage
            .get_starters()
            .await
            .map_err(downcast!(StarterServiceError))
    }

    pub async fn get_starter_by_name(&self, name: &str) -> Result<Starter, StarterServiceError> {
        self.starter_storage
            .get_starter_by_name(name)
            .await
            .map_err(downcast!(StarterServiceError))
    }
}

impl Clone for Box<dyn StarterRepository> {
    fn clone(&self) -> Box<dyn StarterRepository> {
        self.clone_box()
    }
}

unsafe impl Send for StarterService {}
unsafe impl Sync for StarterService {}

pub async fn read_starters_from_fs() -> Result<Vec<Starter>, StarterServiceError> {
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
            },
        };

        starters.push(starter_dto)
    }

    if starters.is_empty() {
        return Err(NoStartersProvided);
    }

    tracing::debug!("Starters: {:?}", starters);
    Ok(starters)
}

fn read_starter(stater_path: PathBuf) -> Result<Starter, StarterServiceError> {
    let starter = cargo_toml::Manifest::from_path(&stater_path)
        .map_err(|_| InvalidStarterManifest(stater_path.clone()))?;
    let package_section = starter
        .package
        .ok_or(MissingPackageSection(stater_path.clone()))?;
    let starter_name = package_section.name().to_string();
    let starter_description = package_section
        .description()
        .unwrap_or("Missing description")
        .to_string();
    let creates = starter.dependencies.into_keys().collect();

    let starter_dto = StarterDto::new(starter_name, creates, starter_description);
    let raw_starter = RawStarter(
        fs::read_to_string(&stater_path).map_err(|_| CannotReadStarterFile(stater_path.clone()))?,
    );

    let starter = Starter {
        starter_dto,
        raw_starter,
    };

    Ok(starter)
}
