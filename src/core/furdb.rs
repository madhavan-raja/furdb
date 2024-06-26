use crate::core::furdb_config::FurDBConfig;

use crate::core::error::FurDBInitializationError;
use std::io::ErrorKind;

#[derive(Clone)]
pub struct FurDB {
    config: FurDBConfig,
}

impl FurDB {
    pub fn new(config: &FurDBConfig) -> Result<Self, FurDBInitializationError> {
        let fur_directory = &config.workdir;

        if !fur_directory.exists() {
            std::fs::create_dir(fur_directory).map_err(|e| match e.kind() {
                ErrorKind::PermissionDenied => FurDBInitializationError::PermissionDenied,
                ErrorKind::NotFound => FurDBInitializationError::InvalidPath,
                _ => FurDBInitializationError::OtherError(e.to_string()),
            })?;
        }

        Ok(Self {
            config: config.to_owned(),
        })
    }

    pub fn get_config(&self) -> FurDBConfig {
        self.config.to_owned()
    }
}
