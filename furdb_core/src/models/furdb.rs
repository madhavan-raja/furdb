use std::error::Error;

use crate::models;

#[derive(Debug, Clone)]
pub struct FurDB {
    config: models::config::Config,
}

impl FurDB {
    pub fn new(config: models::config::Config) -> Result<Self, Box<dyn Error>> {
        let fur_directory = &config.fur_directory;

        if !fur_directory.exists() {
            std::fs::create_dir(&fur_directory)?;
        }

        Ok(Self { config })
    }

    pub fn get_config(&self) -> models::config::Config {
        self.config.to_owned()
    }
}
