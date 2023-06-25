use package_description::PackageDescription;
use serde::{Deserialize, Serialize};
use target_kind::TargetKind;

pub mod package_description;
pub mod target_kind;

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct ProjectDescriptionDto {
    pub target_kind: TargetKind,
    pub package_description: PackageDescription,
    pub starters: Vec<String>,
}
