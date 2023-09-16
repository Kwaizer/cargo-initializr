use std::collections::HashSet;

use cargo_toml::{Dependency, DependencyDetail};
use version_compare::Cmp;

use crate::cargo_toml_parser_extensions::errors::DependencyError;
use crate::cargo_toml_parser_extensions::errors::DependencyError::{
    CouldNotParseDependencies,
    MissingVersionInStarter,
    VersionComparatorError,
};
use crate::cargo_toml_parser_extensions::traits::{Combine, MyToString};
use crate::quote;

impl MyToString for (&String, &Dependency) {
    fn to_string(&self) -> Result<String, DependencyError> {
        let mut result = String::from(self.0);
        result.push_str(" = ");

        match self.1 {
            Dependency::Simple(version) => {
                result.push_str(&quote!('"', version, '"'));
            },
            Dependency::Inherited(_) => {
                unreachable!()
            },
            Dependency::Detailed(details) => {
                result.push_str(&details.to_string()?);
            },
        }

        Ok(result)
    }
}

impl MyToString for DependencyDetail {
    fn to_string(&self) -> Result<String, DependencyError> {
        let parsed_details_toml =
            toml::to_string(&self).map_err(|e| CouldNotParseDependencies(e.to_string()))?;

        let separated_details = parsed_details_toml
            .lines()
            .map(|detail| detail.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        Ok(quote!("{ ", separated_details, " }"))
    }
}

pub fn get_latest_version(v1: &str, v2: &str) -> Result<String, DependencyError> {
    match version_compare::compare(v1, v2)
        .map_err(|_| VersionComparatorError(v1.into(), v2.into()))?
    {
        Cmp::Lt | Cmp::Le => Ok(v2.into()),
        _ => Ok(v1.into()),
    }
}

impl Combine for Dependency {
    fn combine_dependencies(&self, other: &Self) -> Result<Dependency, DependencyError> {
        match self {
            Dependency::Simple(version_a) =>
                match other {
                    Dependency::Simple(version_b) => {
                        let final_version = get_latest_version(version_a, version_b)?;
                        Ok(Dependency::Simple(final_version))
                    },

                    Dependency::Inherited(_) => {
                        unreachable!()
                    },

                    Dependency::Detailed(details) => {
                        let version_b = details.version.as_ref().ok_or(MissingVersionInStarter)?;
                        let final_version = get_latest_version(version_a, version_b)?;

                        let dependency_detail = DependencyDetail {
                            version: Some(final_version),
                            ..details.clone()
                        };

                        Ok(Dependency::Detailed(dependency_detail))
                    },
                },
            Dependency::Inherited(_) => {
                unreachable!()
            },
            Dependency::Detailed(details_a) =>
                match other {
                    Dependency::Simple(version_b) => {
                        let version_a =
                            details_a.version.as_ref().ok_or(MissingVersionInStarter)?;
                        let final_version = get_latest_version(version_a, version_b)?;
                        Ok(Dependency::Simple(final_version))
                    },
                    Dependency::Inherited(_) => {
                        unreachable!()
                    },
                    Dependency::Detailed(details_b) => {
                        let version_a =
                            details_a.version.as_ref().ok_or(MissingVersionInStarter)?;
                        let version_b =
                            details_b.version.as_ref().ok_or(MissingVersionInStarter)?;
                        let final_version = get_latest_version(version_a, version_b)?;

                        let features_list_a = details_a
                            .features
                            .iter()
                            .cloned()
                            .collect::<HashSet<String>>();
                        let features_list_b = details_b
                            .features
                            .iter()
                            .cloned()
                            .collect::<HashSet<String>>();
                        let final_features_list = features_list_a
                            .union(&features_list_b)
                            .cloned()
                            .collect::<Vec<String>>();

                        let dependency_detail = DependencyDetail {
                            version: Some(final_version),
                            features: final_features_list,
                            ..details_a.clone()
                        };

                        Ok(Dependency::Detailed(dependency_detail))
                    },
                },
        }
    }
}
