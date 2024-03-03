use crate::{utils, Column, Database, Table};
use std::error::Error;

impl Database {
    pub fn create_table(
        &self,
        table_id: &str,
        table_name: Option<&str>,
        table_columns: Vec<Column>,
    ) -> Result<Table, Box<dyn Error>> {
        let table_path = utils::get_table_path(&self.database_id, table_id)?;
        let table_config_path = utils::get_table_config_path(&self.database_id, table_id)?;
        let table_data_path = utils::get_table_data_path(&self.database_id, table_id)?;

        let table = Table {
            database_id: self.get_database_id(),
            table_id: String::from(table_id),
            table_name: String::from(table_name.unwrap_or(table_id)),
            table_columns,
        };

        std::fs::create_dir(&table_path)?;
        std::fs::write(&table_config_path, serde_json::to_string(&table)?)?;
        std::fs::write(table_data_path, "")?;

        Ok(table)
    }
}
