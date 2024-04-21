use common::starter::Starter;
use futures::TryFutureExt;
use tracing::log;

use crate::storage::errors::MapStorageError;
use crate::storage::in_memory_storage::InMemoryStorage;
use crate::storage::traits::MapStorage;

#[derive(Clone, Debug)]
pub struct StarterService<T: MapStorage> {
    starter_storage: T,
}

impl<T: MapStorage> StarterService<T> {
    #[allow(dead_code)]
    pub fn new(starter_storage: T) -> Self {
        Self { starter_storage }
    }

    pub async fn get_starters(&self) -> Result<Vec<Starter>, MapStorageError> {
        self.starter_storage
            .get_all_starters()
            .inspect_err(|e| log::error!("{e}"))
            .await
    }

    pub async fn get_starter_by_name(&self, name: &str) -> Result<Starter, MapStorageError> {
        self.starter_storage.get_starter_by_name(name).await
    }
}

impl StarterService<InMemoryStorage> {
    pub fn in_memory() -> Self {
        Self {
            starter_storage: InMemoryStorage::new().expect("Cannot init 'InMemoryStorage'."),
        }
    }
}
