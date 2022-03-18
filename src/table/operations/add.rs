use crate::FurTable;
use bitvec::prelude::*;
use std::{collections::HashMap, error::Error, io::Write};

impl FurTable {
    pub fn add(&mut self, rows: &[HashMap<&str, &str>]) -> Result<(), Box<dyn Error>> {
        let mut rows_bin = BitVec::<u8, Msb0>::new();

        for row in rows {
            let mut row_bin = self.add_row(row)?;
            rows_bin.append(&mut row_bin);
        }

        let bytes: Vec<u8> = rows_bin.into();
        self.write_data(&bytes)?;

        self.data_file_size = Self::get_data_file_size(&self.dir)?;

        Ok(())
    }

    pub(super) fn write_data(&mut self, bytes: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.data_file.write(&bytes)?;

        Ok(())
    }

    pub(super) fn add_row(
        &mut self,
        row: &HashMap<&str, &str>,
    ) -> Result<BitVec<u8, Msb0>, Box<dyn Error>> {
        let mut row_bin = BitVec::new();

        for column in self.table_info.get_columns() {
            let column_id = column.get_id();
            let column_id = column_id.as_str();

            let data = row.get(column_id).unwrap_or(&&"");

            let data_type = column.get_data_type();

            let mut column_bin = data_type.encode(
                data,
                column.get_size(),
                self.table_info.get_converter_server(),
            )?;
            row_bin.append(&mut column_bin);
        }

        Ok(row_bin)
    }
}
