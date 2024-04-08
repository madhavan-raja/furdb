use crate::models;
use crate::utils;
use bitvec::prelude::*;
use std::{error::Error, io::Write};

impl models::table::Table {
    pub fn insert_entries(&self, entries: &[Vec<u128>]) -> Result<(), Box<dyn Error>> {
        let config = self.get_config();
        let table_info = self.get_table_info();

        let mut data = BitVec::<u8, Msb0>::new();

        let table_columns = table_info.get_table_columns();

        for entry in entries {
            assert_eq!(entry.len(), table_columns.len());

            for index in 0..table_columns.len() {
                let element_size = table_columns[index].get_size() as usize;
                let mut element = entry[index];

                let mut current_entry_bin = BitVec::<u8, Msb0>::new();

                assert!(element < 2u128.pow(element_size as u32));

                while element > 0 {
                    current_entry_bin.push(element % 2 == 1);
                    element /= 2;
                }

                while current_entry_bin.len() < element_size {
                    current_entry_bin.push(false);
                }

                current_entry_bin.reverse();

                data.append(&mut current_entry_bin);
            }
        }

        let table_data_path = utils::get_table_data_path(
            &config.fur_directory,
            &table_info.get_database_id(),
            &table_info.get_table_id(),
        );

        let mut table_data_file = std::fs::OpenOptions::new()
            .append(true)
            .open(table_data_path)?;

        table_data_file.write(&Vec::<u8>::from(data))?;

        self.generate_sortfile()?;

        Ok(())
    }
}
