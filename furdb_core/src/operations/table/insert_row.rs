use crate::models;
use crate::utils;
use bitvec::prelude::*;
use std::{error::Error, io::Write};

impl models::table::Table {
    pub fn insert_row(&mut self, row: &[u128]) -> Result<(), Box<dyn Error>> {
        let mut row_bin = BitVec::<u8, Msb0>::new();

        assert_eq!(row.len(), self.table_columns.len());

        for index in 0..self.table_columns.len() {
            let element_size = self.table_columns[index].get_size() as usize;
            let mut element = row[index];

            let mut current_row_bin = BitVec::<u8, Msb0>::new();

            assert!(element < 2u128.pow(element_size as u32));

            while element > 0 {
                current_row_bin.push(element % 2 == 1);
                element /= 2;
            }

            while current_row_bin.len() < element_size {
                current_row_bin.push(false);
            }

            current_row_bin.reverse();

            row_bin.append(&mut current_row_bin);
        }

        let bytes: Vec<u8> = row_bin.into();

        let table_data_path = utils::get_table_data_path(&self.get_database_id(), &self.table_id)?;
        let mut table_data_file = std::fs::OpenOptions::new()
            .append(true)
            .open(table_data_path)?;

        table_data_file.write(&bytes)?;

        Ok(())
    }
}
