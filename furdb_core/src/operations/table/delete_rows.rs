use std::collections::HashSet;
use std::error::Error;

use crate::utils;

impl crate::models::table::Table {
    pub fn delete_rows(&self, indices: Option<Vec<u64>>) -> Result<(), Box<dyn Error>> {
        let database_id = self.get_database_id();
        let table_id = self.get_table_id();

        let data_file_path = utils::get_table_data_path(&database_id, &table_id)?;

        match indices {
            Some(indices) => {
                let table_data_file = std::fs::OpenOptions::new()
                    .read(true)
                    .open(data_file_path)?;

                let data_file_size = table_data_file.metadata().unwrap().len();
                let table_row_size =
                    self.get_table_columns()
                        .iter()
                        .fold(0, |acc, column| acc + column.get_size()) as u64
                        / 8;

                let total_rows = data_file_size / table_row_size;

                let indices = HashSet::<u64>::from_iter(indices);

                let mut remaining_rows = Vec::<Vec<u128>>::new();

                for index in 0..total_rows {
                    if !indices.contains(&index) {
                        remaining_rows.push(self.get_row(index)?);
                    }
                }

                self.delete_rows(None)?;
                self.insert_rows(&remaining_rows)?;
            }
            None => {
                std::fs::write(data_file_path, "")?;
            }
        }

        Ok(())
    }
}
