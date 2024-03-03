use crate::{utils, Column, Database, Table};
use std::error::Error;

impl Database {
    pub fn create_table(
        &self,
        table_id: &str,
        table_name: Option<&str>,
        table_columns: Vec<Column>,
    ) -> Result<Table, Box<dyn Error>> {
        let table_path = utils::get_table_path(&self, table_id)?;
        let table_config_path = utils::get_table_config_path(&self, table_id)?;

        std::fs::create_dir(&table_path)?;

        let table = Table {
            table_id: String::from(table_id),
            table_name: String::from(table_name.unwrap_or(table_id)),
            table_columns,
        };

        std::fs::write(&table_config_path, serde_json::to_string(&table)?)?;

        Ok(table)
    }
}
