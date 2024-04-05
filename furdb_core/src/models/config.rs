use std::{env, error::Error, path::PathBuf};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub fur_directory: PathBuf,
}

impl Config {
    pub fn new(fur_directory: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            fur_directory:  PathBuf::from(fur_directory)
        })
    }
}
