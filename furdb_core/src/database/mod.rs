use crate::DatabaseInfo;
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct Database {
    dir: PathBuf,
    database_info: DatabaseInfo,
}

mod operations;
mod utils;

impl Database {
    pub fn create_database(
        dir: PathBuf,
        database_info: DatabaseInfo,
    ) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir(&dir)?;
        let database = Self { dir, database_info };
        database.save_info()?;

        Ok(())
    }

    pub fn get_database(dir: PathBuf) -> Result<Self, Box<dyn Error>> {
        let database_info = Self::load_info(&dir)?;
        let database = Self { dir, database_info };

        Ok(database)
    }

    pub fn get_info(&self) -> Result<&DatabaseInfo, Box<dyn Error>> {
        Ok(&self.database_info)
    }

    pub fn save_info(&self) -> Result<(), Box<dyn Error>> {
        let db_info_raw = serde_json::to_string(&self.database_info)?;
        let db_info_file_path = Self::get_info_file_path(&self.dir);
        std::fs::write(db_info_file_path, db_info_raw)?;

        Ok(())
    }
}
