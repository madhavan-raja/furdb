use crate::{Database, DatabaseInfo};
use std::{error::Error, path::PathBuf};

impl Database {
    pub fn get_database(dir: PathBuf) -> Result<Self, Box<dyn Error>> {
        let database_info = Self::load_info(&dir)?;
        let database = Self { dir, database_info };

        Ok(database)
    }

    pub fn get_database_info(&self) -> Result<DatabaseInfo, Box<dyn Error>> {
        Ok(self.database_info.clone())
    }
}
