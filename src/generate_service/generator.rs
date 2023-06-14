use crate::cargo_toml_parser_extensions::errors::DependencyError::CouldNotParseDependencies;
use crate::generate_service::traits::Generator;
use crate::project_description_dto::target_kind::TargetKind;
use crate::project_description_dto::ProjectDescriptionDto;
use cargo_toml::{Dependency, DepsSet, Manifest};
use cargo_toml_builder::CargoToml;
use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::path::PathBuf;
use std::{fs, io};
use thiserror::Error;
use tracing_subscriber::fmt::format;

use crate::cargo_toml_parser_extensions::traits::Combine;
use crate::cargo_toml_parser_extensions::traits::MyToString;
use crate::generate_service::generator::ProjectGeneratingError::{
    Directory, Section,
};

pub const MAIN: &str = r#"fn main() {
    println!("Hello, world!");
}"#;

pub const LIB: &str = r#"pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
"#;

#[derive(Error, Debug)]
pub enum ProjectGeneratingError {
    #[error("Could not generate '{0:?}' section: {1:?}")]
    Section(String, String),

    // #[error("Could not generate dependency section: {0:?}")]
    // DependencySectionGeneratingFailure(String),
    #[error("Could not generate directory: {0:?}")]
    Directory(#[from] io::Error),

    #[error("Could not generate '{0:?}' file: {1:?}")]
    File(String, String),
}

#[derive(Clone, Debug)]
pub struct ProjectGenerator<'a> {
    hashed_dir: PathBuf,
    description_dto: &'a ProjectDescriptionDto,
}

impl<'a> ProjectGenerator<'a> {
    pub fn new(project_hash: &String, description_dto: &'a ProjectDescriptionDto) -> Self {
        let mut hashed_dir = PathBuf::from(dotenv::var("TEMP").unwrap());
        hashed_dir.push(project_hash);

        ProjectGenerator {
            hashed_dir,
            description_dto,
        }
    }

    pub fn get_hashed_dir(&self) -> &PathBuf {
        &self.hashed_dir
    }

    fn generate_hashed_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        tracing::info!(
            "Generating 'hashed' directory for: {:?} project",
            self.hashed_dir
        );

        let hashed_dir = self.hashed_dir.clone();
        fs::create_dir(&hashed_dir)?;
        Ok(hashed_dir)
    }

    fn generate_root_dir(&self, hashed_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        tracing::info!(
            "Generating 'root' directory for: {:?} project",
            self.hashed_dir
        );

        let mut root_dir = PathBuf::from(&hashed_dir);
        root_dir.push(&self.description_dto.package_description.name);
        fs::create_dir(&root_dir)?;
        Ok(root_dir)
    }

    fn generate_src_dir(&self, root_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        tracing::info!(
            "Generating 'src' directory for: {:?} project",
            self.hashed_dir
        );

        let mut src_dir = PathBuf::from(root_dir);
        src_dir.push("src");
        fs::create_dir(&src_dir)?;
        Ok(src_dir)
    }

    fn generate_main_file(&self, src_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        tracing::info!(
            "Generating 'main.rs' file for: {:?} project",
            self.hashed_dir
        );

        let mut main_file = PathBuf::from(src_dir);
        main_file.push("main.rs");

        fs::File::create(&main_file)?;
        fs::write(&main_file, MAIN)?;

        Ok(main_file)
    }

    fn generate_lib_file(&self, src_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        tracing::info!(
            "Generating 'lib.rs' file for: {:?} project",
            self.hashed_dir
        );

        let mut lib_file = PathBuf::from(src_dir);
        lib_file.push("lib.rs");

        fs::File::create(&lib_file)?;
        fs::write(&lib_file, LIB)?;

        Ok(lib_file)
    }

    fn generate_cargo_toml_file(&self, root_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        tracing::info!(
            "Generating 'Cargo.toml' file for: {:?} project",
            self.hashed_dir
        );

        let mut cargo_file = PathBuf::from(root_dir);
        cargo_file.push("Cargo.toml");

        fs::File::create(&cargo_file)?;

        let label = format!("{}{}", '#', dotenv::var("LABEL")?);
        let package_section = self.generate_package_section()?;
        let dependency_section = self.generate_dependency_section()?;

        let cargo_file_content = format!("{}{}{}", label, package_section, dependency_section);

        fs::write(&cargo_file, cargo_file_content)?;

        Ok(cargo_file)
    }

    fn generate_dependency_section(&self) -> Result<String, ProjectGeneratingError> {
        let mut starters = Vec::with_capacity(self.description_dto.starters.len());

        for starter in &self.description_dto.starters {
            let starter_content = match self.get_starter_content(starter) {
                Ok(content) => content,
                Err(e) => {
                    tracing::error!("Could not get '{starter}' starter content: {e}");
                    continue;
                }
            };
            starters.push(starter_content);
        }

        let mut parsed_starters = Vec::with_capacity(starters.capacity());
        for starter in starters {
            let starter_content = match Manifest::from_str(&starter) {
                Ok(content) => content,
                Err(e) => {
                    tracing::error!("Could not parse '{starter}' starter content: {e}");
                    continue;
                }
            };
            parsed_starters.push(starter_content);
        }

        let mut dependency_section = String::new();

        let dependency_set = parsed_starters
            .into_iter()
            .map(|manifest| manifest.dependencies)
            .reduce(|starter_a, starter_b| Self::proceed_starters(starter_a, starter_b).unwrap())
            .unwrap();

        dependency_set.iter().for_each(|x| {
            dependency_section.push_str(&x.to_string().unwrap());
            dependency_section.push('\n');
        });

        Ok(dependency_section)
    }

    fn proceed_starters(
        mut starter_a: DepsSet,
        mut starter_b: DepsSet,
    ) -> Result<DepsSet, ProjectGeneratingError> {
        let mut result = BTreeMap::new();

        let dependency_set_a = starter_a.clone().into_keys().collect::<HashSet<String>>();
        let dependency_set_b = starter_b.clone().into_keys().collect::<HashSet<String>>();
        let symmetric_difference = dependency_set_a
            .symmetric_difference(&dependency_set_b)
            .cloned()
            .collect::<Vec<String>>();
        let intersection = dependency_set_a
            .intersection(&dependency_set_b)
            .cloned()
            .collect::<Vec<String>>();

        for dep_name in intersection {
            let dep_a = starter_a.get(&*dep_name).ok_or_else(|| {
                Section(dep_name.clone(), "Could not get dependency".into())
            })?;
            let dep_b = starter_b.get(&*dep_name).ok_or_else(|| {
                Section(dep_name.clone(), "Could not get dependency".into())
            })?;

            let final_dep = dep_a
                .combine_dependencies(dep_b)
                .map_err(|e| Section(dep_name.clone(), e.to_string()))?;
            result.insert(dep_name, final_dep);
        }

        starter_a.append(&mut starter_b);

        for dep_name in symmetric_difference {
            let key_value = starter_a.get_key_value(&*dep_name).ok_or_else(|| {
                Section(dep_name.clone(), "Could not get dependency".into())
            })?;
            result.insert(key_value.0.clone(), key_value.1.clone());
        }

        Ok(result)
    }

    fn generate_package_section(&self) -> Result<String, Box<dyn Error>> {
        let package_section = CargoToml::builder()
            .name(&self.description_dto.package_description.name)
            .version("0.1.0")
            .description(
                &self
                    .description_dto
                    .package_description
                    .description
                    .clone()
                    .unwrap_or_else(|| "No description".to_string()),
            )
            .author(
                &self
                    .description_dto
                    .package_description
                    .author
                    .clone()
                    .unwrap_or_else(|| "Unspecified Author".to_string()),
            )
            .build()?
            .to_string();

        Ok(package_section)
    }

    fn get_starter_content(&self, name: &str) -> Result<String, Box<dyn Error>> {
        let name_with_ext = format!("{}{}", name, ".toml");
        let mut path = PathBuf::from(dotenv::var("CONTENT").unwrap());
        path.push(name_with_ext);

        Ok(fs::read_to_string(path)?)
    }
}

impl<'a> Generator for ProjectGenerator<'a> {
    fn generate_project(&self) -> Result<PathBuf, Box<dyn Error>> {
        let hashed_dir = self.generate_hashed_dir()?;
        let root_dir = self.generate_root_dir(hashed_dir)?;
        let src_dir = self.generate_src_dir(&root_dir)?;

        match self.description_dto.target_kind {
            TargetKind::Bin => self.generate_main_file(&src_dir)?,
            TargetKind::Lib => self.generate_lib_file(&src_dir)?,
        };

        self.generate_cargo_toml_file(&root_dir)?;

        Ok(root_dir)
    }
}

// todo refactor this
impl<'a> Drop for ProjectGenerator<'a> {
    fn drop(&mut self) {
        let hashed_dir = self.hashed_dir.clone();
        tracing::info!("Deleting {:?} project", hashed_dir);
        fs::remove_dir_all(hashed_dir).unwrap_or_default();
    }
}
