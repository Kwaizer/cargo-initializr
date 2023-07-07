use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum TargetKind {
    #[default]
    Bin,
    Lib,
}

impl ToString for TargetKind {
    fn to_string(&self) -> String {
        match self {
            Self::Lib => "Lib".to_string(),
            Self::Bin => "Bin".to_string(),
        }
    }
}

impl FromStr for TargetKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lib" => Ok(Self::Lib),
            "lib" => Ok(Self::Lib),
            "Bin" => Ok(Self::Bin),
            "bin" => Ok(Self::Bin),
            _ => unreachable!(),
        }
    }
}