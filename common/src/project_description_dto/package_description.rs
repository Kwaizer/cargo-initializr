use serde::{Deserialize, Serialize};

const UNNAMED_PROJECT: &str = "unnamed_project";

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageDescription {
    #[serde(default = "default_project_name")]
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
}

impl Default for PackageDescription {
    fn default() -> Self {
        PackageDescription {
            name: UNNAMED_PROJECT.to_string(),
            author: Default::default(),
            description: Default::default()
        }
    }
}

fn default_project_name() -> String {
    UNNAMED_PROJECT.to_string()
}