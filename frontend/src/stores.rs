use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use url::Url;
use yewdux::prelude::Store;

use common::project_description_dto::ProjectDescriptionDto;
use common::starter::starter_dto::StarterDto;

#[derive(Store, Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[store(storage = "session")]
pub struct ProjectDescriptionState {
    pub project_description: ProjectDescriptionDto,
    pub is_description_valid: bool,
}

#[derive(Store, Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[store(storage = "session")]
pub struct StartersState {
    pub all_starters: HashSet<StarterDto>,
    pub selected_starters: HashSet<StarterDto>,
    pub unselected_starters: HashSet<StarterDto>,
}

#[derive(Store, Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub integration_mode: IntegrationMode,
    pub backend_url: Option<Url>,
}

#[derive(Store, Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationMode {
    #[default]
    Test,
    Production,
}

impl From<&str> for IntegrationMode {
    fn from(value: &str) -> Self {
        match value {
            "PRODUCTION" | "production" => Self::Production,
            _ => Self::Test,
        }
    }
}
