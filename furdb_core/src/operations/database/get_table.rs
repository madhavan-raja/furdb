use crate::models;
use crate::utils;
use std::error::Error;

impl models::database::Database {
    pub fn get_table(&self, table_id: &str) -> Result<models::table::Table, Box<dyn Error>> {
        let database_id = self.get_database_id();

        let table_config_path = utils::get_table_config_path(&database_id, table_id)?;
        let table = serde_json::from_reader(std::fs::File::open(&table_config_path)?)?;

        Ok(table)
    }
}
