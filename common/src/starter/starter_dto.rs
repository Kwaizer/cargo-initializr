use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct StarterDto {
    #[validate(length(min = 1))]
    pub name: String,

    pub crates: Vec<String>,

    pub description: String,
}

impl StarterDto {
    pub fn new(name: String, creates: Vec<String>, common_description: String) -> Self {
        StarterDto {
            name,
            crates: creates,
            description: common_description,
        }
    }
}
