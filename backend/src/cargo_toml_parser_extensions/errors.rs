use thiserror::Error;

#[derive(Error, Debug)]
pub enum DependencyError {
    #[error("Could not compare version: {0:?} and {1:?}")]
    VersionComparatorError(String, String),

    #[error("Missing version in starter")]
    MissingVersionInStarter,

    #[error("Could not parse {0:?}")]
    CouldNotParseDependencies(String),
}
