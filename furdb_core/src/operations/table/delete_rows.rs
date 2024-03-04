use std::{
    error::Error,
    io::{Seek, Write},
};

use crate::utils;

impl crate::models::table::Table {
    pub fn delete_rows(&self, indices: Option<Vec<u64>>) -> Result<(), Box<dyn Error>> {
        let database_id = self.get_database_id();
        let table_id = self.get_table_id();

        let data_file_path = utils::get_table_data_path(&database_id, &table_id)?;

        match indices {
            Some(indices) => {
                todo!("Implement delete specific rows");

                // let mut table_data_file = std::fs::OpenOptions::new()
                //     .read(true)
                //     .write(true)
                //     .open(data_file_path)?;

                // let row_size =
                //     self.get_table_columns()
                //         .iter()
                //         .fold(0, |acc, column| acc + column.get_size()) as u64
                //         / 8;

                // for index in indices {
                //     table_data_file.seek(std::io::SeekFrom::Start(index * row_size))?;
                //     table_data_file.write_all(&vec![0u8; row_size as usize])?;
                // }
            }
            None => {
                std::fs::write(data_file_path, "")?;
            }
        }

        Ok(())
    }
}
