use crate::project_description_dto::target_kind::TargetKind;
use crate::push;
use crate::service::project::ProjectError::{NoSuchFile, WritingToFile};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};
use thiserror::Error;

pub enum ProjectFileTarget {
    Main,
    Lib,
    Cargo,
}

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("Could not generate project: {0:?}")]
    ProjectCreationFailure(#[from] io::Error),

    #[error("Could not write to {0:?} file: {1:?}")]
    WritingToFile(String, String),

    #[error("This file doesn't exists")]
    NoSuchFile,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Project {
    hashed_dir: PathBuf,
    root_dir: PathBuf,
    src_dir: PathBuf,
    main_file: Option<File>,
    lib_file: Option<File>,
    cargo_file: File,
}

impl Project {
    pub fn new(
        project_hash: &str,
        project_name: &str,
        target_kind: &TargetKind,
    ) -> Result<Project, ProjectError> {
        let hashed_dir = push!(PathBuf::from(dotenv::var("TEMP").unwrap()), project_hash);
        fs::create_dir(&hashed_dir)?;

        let root_dir = push!(hashed_dir, project_name);
        fs::create_dir(&root_dir)?;

        let src_dir = push!(root_dir, "src");
        fs::create_dir(&src_dir)?;

        let main_file = match target_kind {
            TargetKind::Bin => {
                let main_file_path = push!(src_dir, "main.rs");
                Some(File::create(&main_file_path)?)
            }
            TargetKind::Lib => None,
        };

        let lib_file = match target_kind {
            TargetKind::Bin => None,
            TargetKind::Lib => {
                let lib_file_path = push!(src_dir, "lib.rs");
                Some(File::create(&lib_file_path)?)
            }
        };

        let cargo_file_path = push!(root_dir, "Cargo.toml");
        let cargo_file = File::create(&cargo_file_path)?;

        let project = Project {
            hashed_dir,
            root_dir,
            src_dir,
            main_file,
            lib_file,
            cargo_file,
        };

        Ok(project)
    }

    pub fn write_to_file(
        &mut self,
        content: &str,
        file: ProjectFileTarget,
    ) -> Result<(), ProjectError> {
        match file {
            ProjectFileTarget::Main => {
                self.main_file
                    .as_mut()
                    .ok_or(NoSuchFile)?
                    .write_all(content.as_bytes())
                    .map_err(|e| WritingToFile("lib".into(), e.to_string()))?;
            }
            ProjectFileTarget::Lib => {
                self.lib_file
                    .as_mut()
                    .ok_or(NoSuchFile)?
                    .write_all(content.as_bytes())
                    .map_err(|e| WritingToFile("lib".into(), e.to_string()))?;
            }
            ProjectFileTarget::Cargo => {
                self.cargo_file
                    .write_all(content.as_bytes())
                    .map_err(|e| WritingToFile("lib".into(), e.to_string()))?;
            }
        };

        Ok(())
    }

    pub fn get_hashed_dir_path(&self) -> PathBuf {
        self.hashed_dir.clone()
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        tracing::info!("Deleting {:?} project", self.hashed_dir);
        fs::remove_dir_all(&self.hashed_dir).unwrap_or_default();
    }
}
