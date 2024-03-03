use crate::{utils, Database, Table};
use std::error::Error;

impl Database {
    pub fn get_all_tables(&self) -> Result<Vec<Table>, Box<dyn Error>> {
        let all_tables_path = utils::get_all_tables_path(&self.database_id)?;

        let all_table_id = std::fs::read_dir(all_tables_path)?
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<String>>();

        let mut all_tables = Vec::new();
        for table_id in all_table_id {
            let table = self.get_table(&table_id)?;
            all_tables.push(table);
        }

        Ok(all_tables)
    }
}
