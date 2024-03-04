use std::{env, error::Error, path::PathBuf};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub fur_directory: PathBuf,
}

impl Config {
    pub fn new(fur_directory: Option<&str>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            fur_directory: match fur_directory {
                Some(fur_directory) => PathBuf::from(fur_directory),
                None => PathBuf::from(env::var("FUR_DIRECTORY")?),
            },
        })
    }
}
