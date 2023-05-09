use std::error::Error;
use std::path::PathBuf;

pub trait Generator {
    fn generate_project(&self) -> Result<PathBuf, Box<dyn Error>>;
}
