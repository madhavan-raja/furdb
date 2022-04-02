use crate::FurDBInfo;
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct FurDB {
    dir: PathBuf,
    db_info: FurDBInfo,
}

mod operations;
mod utils;

impl FurDB {
    pub fn new(dir: PathBuf, db_info: Option<FurDBInfo>) -> Result<Self, Box<dyn Error>> {
        Self::ensure_db_files(&dir)?;

        let db_info = if db_info.is_some() {
            db_info.unwrap()
        } else {
            Self::load_info(&dir)?
        };

        Ok(Self { dir, db_info })
    }

    pub fn get_info(&self) -> Result<&FurDBInfo, Box<dyn Error>> {
        Ok(&self.db_info)
    }

    pub fn save_info(&self) -> Result<(), Box<dyn Error>> {
        let db_info_raw = serde_json::to_string(&self.db_info)?;
        let db_info_file_path = Self::get_info_file_path(&self.dir);
        std::fs::write(db_info_file_path, db_info_raw)?;

        Ok(())
    }
}
