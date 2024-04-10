use crate::core::errors::furdb_errors::furdb_initialization_error::FurDBInitializationError;
use std::io::ErrorKind;

use crate::core::models;

#[derive(Debug, Clone)]
pub struct FurDB {
    config: models::config::Config,
}

impl FurDB {
    pub fn new(config: &models::config::Config) -> Result<Self, FurDBInitializationError> {
        let fur_directory = &config.fur_directory;

        if !fur_directory.exists() {
            std::fs::create_dir(&fur_directory).map_err(|e| match e.kind() {
                ErrorKind::PermissionDenied => FurDBInitializationError::PermissionDenied,
                ErrorKind::NotFound => FurDBInitializationError::InvalidPath,
                _ => FurDBInitializationError::OtherError(e.to_string()),
            })?;
        }

        Ok(Self {
            config: config.to_owned(),
        })
    }

    pub fn get_config(&self) -> models::config::Config {
        self.config.to_owned()
    }
}
