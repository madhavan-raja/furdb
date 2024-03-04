use crate::models;
use crate::utils;
use std::error::Error;

impl models::furdb::FurDB {
    pub fn get_database(
        &self,
        database_id: &str,
    ) -> Result<models::database::Database, Box<dyn Error>> {
        let config = self.get_config();

        let database_config_path =
            utils::get_database_config_path(&config.fur_directory, database_id)?;

        let database_info = serde_json::from_reader(std::fs::File::open(&database_config_path)?)?;
        let database = models::database::Database::new(&config, &database_info)?;

        Ok(database)
    }
}
