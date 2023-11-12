use crate::starter::raw_starter::RawStarter;
use crate::starter::starter_dto::StarterDto;
use serde::{Deserialize, Serialize};

pub mod raw_starter;
pub mod starter_dto;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Starter {
    pub starter_dto: StarterDto,
    pub raw_starter: RawStarter,
}
