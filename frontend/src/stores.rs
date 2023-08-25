use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use yewdux::prelude::Store;

use common::project_description_dto::ProjectDescriptionDto;
use common::starter_dto::StarterDto;

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
