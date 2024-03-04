use crate::models;
use crate::utils;
use std::error::Error;

impl models::database::Database {
    pub fn get_table(&self, table_id: &str) -> Result<models::table::Table, Box<dyn Error>> {
        let config = self.get_config();
        let database_info = self.get_database_info();

        let table_config_path = utils::get_table_config_path(
            &config.fur_directory,
            &database_info.get_database_id(),
            table_id,
        )?;

        let table_info = serde_json::from_reader(std::fs::File::open(&table_config_path)?)?;
        let table = models::table::Table::new(&config, &table_info)?;

        Ok(table)
    }
}
