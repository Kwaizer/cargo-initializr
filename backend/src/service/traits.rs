use std::error::Error;

use async_trait::async_trait;
use common::starter::Starter;

type T = Starter;

#[async_trait]
pub trait StarterRepository: StartersSupplier + StarterRepositoryCloneable {
    async fn load_starters(self: &mut Self) -> Result<(), Box<dyn Error>>;

    async fn get_starters(self: &Self) -> Result<Vec<T>, Box<dyn Error>>;

    async fn get_starter_by_name(self: &Self, name: &str) -> Result<T, Box<dyn Error>>;
}

pub trait StarterRepositoryCloneable {
    fn clone_box(&self) -> Box<dyn StarterRepository>;
}

impl<T> StarterRepositoryCloneable for T
where
    T: 'static + StarterRepository + Clone,
{
    fn clone_box(&self) -> Box<dyn StarterRepository> {
        Box::new(self.clone())
    }
}

#[async_trait]
pub trait StartersSupplier {
    async fn init_starters(self: &Self) -> Result<Vec<T>, Box<dyn Error>>;
}

pub trait StarterReaderSync {
    fn read_starter_content<ToString>(starter_name: ToString) -> Result<String, Box<dyn Error>>
    where
        ToString: Into<String>;
}
