use crate::{utils, Database, Table};
use std::error::Error;

impl Database {
    pub fn get_all_tables(&self) -> Result<Vec<Table>, Box<dyn Error>> {
        let all_tables_path = utils::get_all_tables_path(&self.database_id)?;

        return Ok(Vec::new());

        todo!("Implement getting all tables from the database");

        // let all_tables = std::fs::read_dir(all_tables_path)?
        //     .map(|_| Table {
        //         table_id: String::from("A"),
        //         table_name: String::from("B"),
        //         table_columns: Vec::new(),
        //     })
        //     .collect();

        // let tables = Vec::new();

        // let table = serde_json::from_reader(std::fs::File::open(&table_config_path)?)?;

        // Ok(tables)
    }
}
