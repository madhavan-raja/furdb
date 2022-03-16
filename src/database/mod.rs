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
    pub fn new(dir: PathBuf, db_info: Option<FurDBInfo>) -> Result<FurDB, Box<dyn Error>> {
        Self::ensure_db_files(&dir)?;

        let db_info = if db_info.is_some() {
            db_info.unwrap()
        } else {
            Self::load_info(&dir)?
        };

        Ok(FurDB { dir, db_info })
    }

    pub fn get_info(&self) -> Result<&FurDBInfo, Box<dyn Error>> {
        Ok(&self.db_info)
    }
}
