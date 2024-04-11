use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FurDBConfig {
    pub fur_directory: PathBuf,
}

impl FurDBConfig {
    pub fn new(fur_directory: &str) -> Self {
        Self {
            fur_directory: PathBuf::from(fur_directory),
        }
    }
}
