use crate::core::errors::entry_errors::entry_insertion_error::EntryInsertionError;

use bitvec::prelude::*;
use std::io::Write;

use crate::core::models;
use crate::core::utils;

impl models::table::Table {
    pub fn generate_sortfile(&self) -> Result<(), EntryInsertionError> {
        let config = self.get_config();
        let table_info = self.get_table_info();

        let all_entries = self
            .get_entries(None)
            .map_err(|e| EntryInsertionError::OtherError(e.to_string()))?
            .get_results();
        let identifier_size = if all_entries.len() > 0 {
            (1 + ((all_entries.len() - 1) / 256)) * 8
        } else {
            0
        };

        let mut data = BitVec::<u8, Msb0>::new();

        let table_columns = table_info.get_table_columns();

        for column_index in 0..table_columns.len() {
            let mut sortfile_column = all_entries
                .iter()
                .enumerate()
                .map(|(entry_index, entry)| (entry_index, entry.get_data()[column_index]))
                .collect::<Vec<(usize, u128)>>();

            sortfile_column.sort_by(|a, b| a.1.cmp(&b.1));

            sortfile_column.iter().for_each(|(entry_index, _)| {
                let mut entry_index = entry_index.to_owned() as u64;
                let mut current_sortfile_column_bin = BitVec::<u8, Msb0>::new();

                while entry_index > 0 {
                    current_sortfile_column_bin.push(entry_index % 2 == 1);
                    entry_index /= 2;
                }

                while current_sortfile_column_bin.len() < identifier_size {
                    current_sortfile_column_bin.push(false);
                }

                current_sortfile_column_bin.reverse();

                data.append(&mut current_sortfile_column_bin);
            });
        }

        let table_sortfile_path = utils::get_sortfile_path(
            &config.fur_directory,
            &table_info.get_database_id(),
            &table_info.get_table_id(),
        );

        let mut table_sortfile = std::fs::OpenOptions::new()
            .write(true)
            .open(&table_sortfile_path)
            .map_err(|e| EntryInsertionError::OtherError(e.to_string()))?;

        std::fs::write(table_sortfile_path, "")
            .map_err(|e| EntryInsertionError::OtherError(e.to_string()))?;
        table_sortfile
            .write(&Vec::<u8>::from(data))
            .map_err(|e| EntryInsertionError::OtherError(e.to_string()))?;

        Ok(())
    }
}
