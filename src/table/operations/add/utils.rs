use crate::FurTable;
use bitvec::prelude::*;
use std::{collections::HashMap, error::Error, io::Write};

impl FurTable {
    pub(crate) fn write_data(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        self.data_file.write(&bytes)?;
        self.data_file_size = Self::get_data_file_size(&self.dir)?;

        Ok(())
    }

    pub(crate) fn convert_row_to_bin(
        &self,
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
