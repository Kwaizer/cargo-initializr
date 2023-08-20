use package_description::PackageDescription;
use serde::{Deserialize, Serialize};
use target_kind::TargetKind;
use crate::starter_dto::StarterDto;

pub mod package_description;
pub mod target_kind;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Default, Eq, PartialEq)]
pub struct ProjectDescriptionDto {
    pub target_kind: TargetKind,
    pub package_description: PackageDescription,
    pub starters: Vec<String>,
}
