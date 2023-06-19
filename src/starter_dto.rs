use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StarterDto {
    pub name: String,
    pub crates: Vec<String>,
    pub common_description: String,
}

impl StarterDto {
    pub fn new(name: String, creates: Vec<String>, common_description: String) -> Self {
        StarterDto {
            name,
            crates: creates,
            common_description,
        }
    }
}
