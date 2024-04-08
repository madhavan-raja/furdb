use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub fur_directory: PathBuf,
}

impl Config {
    pub fn new(fur_directory: &str) -> Self {
        Self {
            fur_directory:  PathBuf::from(fur_directory)
        }
    }
}
