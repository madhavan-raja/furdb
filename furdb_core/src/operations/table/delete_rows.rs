use std::collections::HashSet;
use std::error::Error;

use crate::utils;

impl crate::models::table::Table {
    pub fn delete_rows(&self, indices: Option<Vec<u64>>) -> Result<(), Box<dyn Error>> {
        let config = self.get_config();
        let table_info = self.get_table_info();

        let data_file_path = utils::get_table_data_path(
            &config.fur_directory,
            &table_info.get_database_id(),
            &table_info.get_table_id(),
        )?;

        let table_data_file = std::fs::OpenOptions::new()
            .read(true)
            .open(&data_file_path)?;

        let data_file_size = table_data_file.metadata()?.len();
        let table_row_size = table_info
            .get_table_columns()
            .iter()
            .fold(0, |acc, column| acc + column.get_size()) as u64
            / 8;

        let total_rows = data_file_size / table_row_size;

        match indices {
            Some(indices) => {
                let indices = HashSet::<u64>::from_iter(indices);

                let remaining_rows = (0..total_rows)
                    .filter(|index| !indices.contains(index))
                    .map(|index| self.get_row(index))
                    .collect::<Result<Vec<Vec<u128>>, Box<dyn Error>>>()?;

                self.delete_rows(None)?;
                self.insert_rows(&remaining_rows)?;
            }
            None => {
                std::fs::write(&data_file_path, "")?;
            }
        }

        self.generate_sortfile()?;

        Ok(())
    }
}
