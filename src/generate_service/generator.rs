use crate::generate_service::traits::Generator;
use crate::project_description_dto::target_kind::TargetKind;
use crate::project_description_dto::ProjectDescriptionDto;
use cargo_toml_builder::CargoToml;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

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

#[derive(Clone, Debug)]
pub struct ProjectGenerator<'a> {
    hashed_dir: PathBuf,
    description_dto: &'a ProjectDescriptionDto,
}

impl<'a> ProjectGenerator<'a> {
    pub fn new(project_hash: &String, description_dto: &'a ProjectDescriptionDto) -> Self {
        let path = format!("{}{}{}", dotenv::var("TEMP").unwrap(), "/", project_hash);
        let hashed_dir = PathBuf::from(path);

        ProjectGenerator {
            hashed_dir,
            description_dto,
        }
    }

    pub fn get_hashed_dir(&self) -> PathBuf {
        self.hashed_dir.clone()
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
        let content = CargoToml::builder()
            .name("test")
            .version("0.1.0")
            .author("Alice Smith <asmith@example.com>")
            .build()
            .unwrap()
            .to_string();
        fs::write(&cargo_file, content)?;

        Ok(cargo_file)
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
