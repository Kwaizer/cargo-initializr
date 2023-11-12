use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Eq, PartialEq, Debug, Error, Serialize, Deserialize)]
pub enum MapStorageError {
    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Failed to deserialize value: {0}")]
    DeserializeFailed(String),

    #[error("Failed to serialize value: {0}")]
    SerializeFailed(String),

    #[error("Key '{0}' is not found")]
    KeyNotFound(String),
}

#[derive(Error, Debug)]
pub enum StarterFileError {
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
