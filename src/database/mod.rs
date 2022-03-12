use crate::FurDBInfo;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FurDB {
    dir: PathBuf,
}

mod operations;
mod utils;

impl FurDB {
    pub fn new(dir: PathBuf, db_info: Option<FurDBInfo>) -> std::io::Result<FurDB> {
        Self::ensure_db_files(&dir, db_info)?;
        Ok(FurDB { dir })
    }

    pub fn get_info(&self) -> std::io::Result<FurDBInfo> {
        let db_info_file_path = Self::get_info_file_path(&self.dir);
        let db_info_contents_raw = std::fs::read_to_string(&db_info_file_path)?;
        let db_info_contents = serde_json::from_str(&db_info_contents_raw)?;
        let db_info = serde_json::from_value(db_info_contents)?;

        Ok(db_info)
    }
}
