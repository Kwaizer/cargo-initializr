use cargo_toml::Dependency;

use crate::cargo_toml_parser_extensions::errors::DependencyError;

pub trait MyToString {
    fn to_string(&self) -> Result<String, DependencyError>;
}

pub trait Combine {
    fn combine_dependencies(&self, other: &Self) -> Result<Dependency, DependencyError>;
}
