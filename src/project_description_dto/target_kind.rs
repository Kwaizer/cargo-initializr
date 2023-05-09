use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub enum TargetKind {
    Bin,
    Lib,
}
