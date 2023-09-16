use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;
use common::starter::Starter;
use thiserror::Error;

use crate::repository::hash_map_starter_repository::ProjectGeneratingServiceError::CannotFindStarter;
use crate::service::starter_service::read_starters_from_fs;
use crate::service::traits::{StarterRepository, StartersSupplier};

#[derive(Error, Debug)]
pub enum ProjectGeneratingServiceError {
    #[error("Cannot find starter: {0:?}")]
    CannotFindStarter(String),
}

#[async_trait]
impl StartersSupplier for HashMap<String, Starter> {
    async fn init_starters(&self) -> Result<Vec<Starter>, Box<dyn Error>> {
        read_starters_from_fs()
            .await
            .map_err(|e| Box::new(e).into())
    }
}

#[async_trait]
impl StarterRepository for HashMap<String, Starter> {
    async fn load_starters(&mut self) -> Result<(), Box<dyn Error>> {
        let starters = self.init_starters().await?;
        for starter in starters {
            self.insert(starter.starter_dto.name.clone(), starter);
        }

        Ok(())
    }

    async fn get_starters(&self) -> Result<Vec<Starter>, Box<dyn Error>> {
        Ok(self.clone().into_values().collect())
    }

    async fn get_starter_by_name(&self, name: &str) -> Result<Starter, Box<dyn Error>> {
        self.get(name)
            .cloned()
            .ok_or(Box::new(CannotFindStarter(name.to_owned())).into())
    }
}
