use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Default,  Eq, PartialEq, Serialize, Deserialize)]
pub enum TargetKind {
    #[default]
    Bin,
    Lib,
}
