use crate::FurTable;
use bitvec::prelude::*;
use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Seek, SeekFrom},
};

impl FurTable {
    pub fn get_row_bin(
        &mut self,
        index: u64,
    ) -> Result<HashMap<String, BitVec<u8, Msb0>>, Box<dyn Error>> {
        let mut result = HashMap::<String, BitVec<u8, Msb0>>::new();

        let row_size = self.get_row_size()? / 8;

        let row_start = index * row_size as u64;

        self.data_file.seek(SeekFrom::Start(row_start))?;

        let mut buf = vec![0u8; row_size];

        self.data_file.read_exact(&mut buf)?;
        let row_bin: BitVec<u8, Msb0> = BitVec::from_slice(&buf);

        let mut column_start = 0;
        for column in self.table_info.get_columns() {
            let column_size = column.get_size() as usize;

            let data_bin = &row_bin[column_start..(column_start + column_size)];
            let data_bin = BitVec::from(data_bin);
            column_start += column_size;

            result.insert(column.get_id(), data_bin);
        }

        Ok(result)
    }

    pub fn get_rows_bin(
        &mut self,
        indices: Vec<u64>,
    ) -> Result<Vec<HashMap<String, BitVec<u8, Msb0>>>, Box<dyn Error>> {
        let mut results = Vec::<HashMap<String, BitVec<u8, Msb0>>>::new();

        for index in indices {
            let result = self.get_row_bin(index)?;

            results.push(result);
        }

        Ok(results)
    }

    pub fn get_bin(&mut self) -> Result<Vec<HashMap<String, BitVec<u8, Msb0>>>, Box<dyn Error>> {
        let row_size = self.get_row_size()? / 8;

        let indices: Vec<u64> = (0..self.data_file_size / row_size as u64).collect();

        let results = self.get_rows_bin(indices)?;

        Ok(results)
    }

    pub fn get(&mut self) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
        let rows_bin = self.get_bin()?;
        let mut results = Vec::<HashMap<String, String>>::new();

        let table_info = self.get_info()?;

        for row_bin in rows_bin {
            let mut row = HashMap::new();

            for column in table_info.get_columns() {
                let data_type = column.get_data_type();

                let data_bin = row_bin.get(&column.get_id()).unwrap();
                let data = data_type.decode(data_bin, table_info.get_converter_server())?;

                row.insert(column.get_id(), data);
            }

            results.push(row);
        }

        Ok(results)
    }
}
