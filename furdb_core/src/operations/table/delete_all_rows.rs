use std::error::Error;

use crate::utils;

impl crate::models::table::Table {
    pub fn delete_all_rows(&self) -> Result<(), Box<dyn Error>> {
        let database_id = self.get_database_id();
        let table_id = self.get_table_id();

        let data_file_path = utils::get_table_data_path(&database_id, &table_id)?;
        std::fs::write(data_file_path, "")?;

        Ok(())
    }
}
