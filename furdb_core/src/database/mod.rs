use crate::DatabaseInfo;
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct Database {
    dir: PathBuf,
    db_info: DatabaseInfo,
}

mod operations;
mod utils;

impl Database {
    pub fn new(dir: PathBuf, db_info: Option<DatabaseInfo>) -> Result<Self, Box<dyn Error>> {
        Self::ensure_db_files(&dir)?;

        let db_info = if db_info.is_some() {
            db_info.unwrap()
        } else {
            Self::load_info(&dir)?
        };

        let db = Self { dir, db_info };

        db.save_info()?;

        Ok(db)
    }

    pub fn get_info(&self) -> Result<&DatabaseInfo, Box<dyn Error>> {
        Ok(&self.db_info)
    }

    pub fn save_info(&self) -> Result<(), Box<dyn Error>> {
        let db_info_raw = serde_json::to_string(&self.db_info)?;
        let db_info_file_path = Self::get_info_file_path(&self.dir);
        std::fs::write(db_info_file_path, db_info_raw)?;

        Ok(())
    }
}
