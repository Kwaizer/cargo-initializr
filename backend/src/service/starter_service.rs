use common::starter::Starter;
use futures::TryFutureExt;
use tracing::log;

use crate::storage::errors::MapStorageError;
use crate::storage::traits::MapStorage;

#[derive(Clone, Debug)]
pub struct StarterService<T: MapStorage> {
    starter_storage: T,
}

impl<T: MapStorage> StarterService<T> {
    pub fn new(starter_storage: T) -> Self {
        Self { starter_storage }
    }

    pub async fn get_starters(&self) -> Result<Vec<Starter>, MapStorageError> {
        self.starter_storage
            .get_all_starters()
            .map_err(|e| {
                log::error!("{e}");
                e
            })
            .await
    }

    pub async fn get_starter_by_name(&self, name: &str) -> Result<Starter, MapStorageError> {
        self.starter_storage.get_starter_by_name(name).await
    }
}
