use crate::models;
use crate::utils;
use std::error::Error;

impl models::database::Database {
    pub fn delete_table(&self, table_id: &str) -> Result<(), Box<dyn Error>> {
        let config = self.get_config();
        let database_info = self.get_database_info();

        let table_path = utils::get_table_path(
            &config.fur_directory,
            &database_info.get_database_id(),
            table_id,
        );

        std::fs::remove_dir_all(&table_path)?;

        Ok(())
    }
}
