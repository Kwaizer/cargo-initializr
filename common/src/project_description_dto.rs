use package_description::PackageDescription;
use serde::{Deserialize, Serialize};
use target_kind::TargetKind;

pub mod package_description;
pub mod target_kind;

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Default, Eq, PartialEq)]
pub struct ProjectDescriptionDto {
    pub target_kind: TargetKind,
    pub package_description: PackageDescription,
    pub starters: Vec<String>,
}
