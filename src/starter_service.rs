use crate::starter_service::StarterServiceError::StripSuffix;
use std::error::Error;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum StarterServiceError {
    #[error("Could not strip suffix")]
    StripSuffix,
}

pub fn get_starters() -> Result<Vec<String>, Box<dyn Error>> {
    let content = dotenv::var("CONTENT")?;
    let mut starters = Vec::new();

    for file in WalkDir::new(content)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata()?.is_dir() {
            continue;
        }

        let starter_name = match file.file_name().to_str() {
            Some(name) => name.strip_suffix(".toml").ok_or(StripSuffix)?.to_string(),
            None => {
                tracing::error!("Could not read name of file: {:?}", file);
                continue;
            }
        };

        starters.push(starter_name)
    }

    tracing::debug!("Starters: {:?}", starters);
    Ok(starters)
}
