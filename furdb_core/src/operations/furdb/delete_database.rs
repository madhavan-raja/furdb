use crate::models;
use crate::utils;
use std::error::Error;

impl models::furdb::FurDB {
    pub fn delete_database(&self, database_id: &str) -> Result<(), Box<dyn Error>> {
        let config = self.get_config();

        let database_path = utils::get_database_path(&config.fur_directory, database_id)?;

        std::fs::remove_dir_all(&database_path)?;

        Ok(())
    }
}
