use std::{error::Error, path::PathBuf};

use crate::{Database, DatabaseInfo};

impl Database {
    pub(crate) fn get_info_file_path(dir: &PathBuf) -> PathBuf {
        let mut db_info_file_path = dir.clone();
        db_info_file_path.push("fur.json");

        db_info_file_path
    }

    pub(crate) fn ensure_db_files(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        if !dir.exists() {
            std::fs::create_dir(&dir)?;
        }

        Ok(())
    }

    pub(crate) fn load_info(dir: &PathBuf) -> Result<DatabaseInfo, Box<dyn Error>> {
        let db_info_file_path = Self::get_info_file_path(&dir);
        let db_info_contents_raw = std::fs::read_to_string(&db_info_file_path)?;
        let db_info_contents = serde_json::from_str(&db_info_contents_raw)?;
        let db_info = serde_json::from_value(db_info_contents)?;

        Ok(db_info)
    }
}
