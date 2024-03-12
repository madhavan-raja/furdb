use bitvec::prelude::*;
use std::{error::Error, io::Write};

use crate::{models, utils};

impl models::table::Table {
    pub(crate) fn generate_sortfile(&self) -> Result<(), Box<dyn Error>> {
        let config = self.get_config();
        let table_info = self.get_table_info();

        let all_rows = self.get_rows(None)?.get_results();
        let identifier_size = if all_rows.len() > 0 {
            (1 + ((all_rows.len() - 1) / 256)) * 8
        } else {
            0
        };

        let mut data = BitVec::<u8, Msb0>::new();

        let table_columns = table_info.get_table_columns();

        for column_index in 0..table_columns.len() {
            let mut sortfile_column = all_rows
                .iter()
                .enumerate()
                .map(|(row_index, row)| (row_index, row.get_data()[column_index]))
                .collect::<Vec<(usize, u128)>>();

            sortfile_column.sort_by(|a, b| a.1.cmp(&b.1));

            sortfile_column.iter().for_each(|(row_index, _)| {
                let mut row_index = row_index.to_owned() as u64;
                let mut current_sortfile_column_bin = BitVec::<u8, Msb0>::new();

                while row_index > 0 {
                    current_sortfile_column_bin.push(row_index % 2 == 1);
                    row_index /= 2;
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
        )?;

        let mut table_sortfile = std::fs::OpenOptions::new()
            .write(true)
            .open(&table_sortfile_path)?;

        std::fs::write(table_sortfile_path, "")?;
        table_sortfile.write(&Vec::<u8>::from(data))?;

        Ok(())
    }
}
