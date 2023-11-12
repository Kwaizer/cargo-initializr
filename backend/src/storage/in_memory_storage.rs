use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use async_trait::async_trait;
use common::starter::raw_starter::RawStarter;
use common::starter::starter_dto::StarterDto;
use common::starter::Starter;
use walkdir::WalkDir;

use crate::storage::errors::StarterFileError::*;
use crate::storage::errors::{MapStorageError, StarterFileError};
use crate::storage::traits::MapStorage;

#[derive(Clone, Debug)]
pub struct InMemoryStorage(HashMap<String, Starter>);

impl InMemoryStorage {
    pub fn new() -> Result<Self, StarterFileError> {
        Ok(Self(HashMap::from_iter(
            read_starters_from_fs()?
                .into_iter()
                .map(|s| (s.starter_dto.name.clone(), s)),
        )))
    }
}

#[async_trait]
impl MapStorage for InMemoryStorage {
    async fn get_all_starters(self: &Self) -> Result<Vec<Starter>, MapStorageError> {
        Ok(self.0.values().cloned().collect())
    }

    async fn get_starter_by_name<N>(self: &Self, name: N) -> Result<Starter, MapStorageError>
    where
        N: Into<String> + Send,
    {
        let name = name.into();
        self.0
            .get(&name)
            .cloned()
            .ok_or(MapStorageError::KeyNotFound(name))
    }
}

fn read_starters_from_fs() -> Result<Vec<Starter>, StarterFileError> {
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

fn read_starter(stater_path: PathBuf) -> Result<Starter, StarterFileError> {
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
