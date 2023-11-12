use async_trait::async_trait;
use common::starter::Starter;

use crate::storage::errors::MapStorageError;

#[async_trait]
pub trait MapStorage {
    async fn get_all_starters(self: &Self) -> Result<Vec<Starter>, MapStorageError>;

    // async fn get_starter_by_name<N, S>(self: &Self, name: N) -> Result<S, Box<MapStorageError>>
    //     where
    //         N: serde::de::DeserializeOwned + Send,
    //         S: Into<String> + Send;

    async fn get_starter_by_name<N>(self: &Self, name: N) -> Result<Starter, MapStorageError>
    where
        N: Into<String> + Send;

    // async fn load_starters(&mut self) -> Result<(), Box<Error>>;
}

// pub trait StarterRepositoryCloneable {
//     fn clone_box(&self) -> Box<dyn StarterRepository>;
// }
//
// impl<T> StarterRepositoryCloneable for T
//     where
//         T: 'static + StarterRepository + Clone,
// {
//     fn clone_box(&self) -> Box<dyn StarterRepository> {
//         Box::new(self.clone())
//     }
// }
//
// #[async_trait]
// pub trait StartersSupplier {
//     async fn init_starters(self: &Self) -> Result<Vec<T>, Box<dyn Error>>;
// }
//
// pub trait StarterReaderSync {
//     fn read_starter_content<ToString>(starter_name: ToString) -> Result<String, Box<dyn Error>>
//         where
//             ToString: Into<String>;
// }
