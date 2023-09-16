use serde::{Deserialize, Serialize};
use crate::starter::raw_starter::RawStarter;
use crate::starter::starter_dto::StarterDto;

pub mod starter_dto;
pub mod raw_starter;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Starter {
    pub starter_dto: StarterDto,
    pub raw_starter: RawStarter,
}