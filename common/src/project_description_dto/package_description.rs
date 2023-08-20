use std::ops::Deref;
use serde::{Deserialize, Serialize};

const UNNAMED_PROJECT: &str = "unnamed_project";

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct PackageDescription {
    pub name: PackageDescriptionName,

    pub author: Option<String>,

    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PackageDescriptionName(pub String);

impl Deref for PackageDescriptionName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(clippy::from_over_into)]
impl Into<PackageDescriptionName> for String {
    fn into(self) -> PackageDescriptionName {
        PackageDescriptionName(self)
    }
}

impl Default for PackageDescriptionName {
    fn default() -> Self {
        PackageDescriptionName(UNNAMED_PROJECT.to_string())
    }
}