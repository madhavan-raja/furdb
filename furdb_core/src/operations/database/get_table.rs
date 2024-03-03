use crate::{utils, Database, Table};
use std::error::Error;

impl Database {
    pub fn get_table(&self, table_id: &str) -> Result<Table, Box<dyn Error>> {
        let table_config_path = utils::get_table_config_path(&self.database_id, table_id)?;
        let table = serde_json::from_reader(std::fs::File::open(&table_config_path)?)?;

        Ok(table)
    }
}
