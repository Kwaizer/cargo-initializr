use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct PackageDescription {
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
}
