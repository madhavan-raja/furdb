use crate::models;
use crate::utils;
use bitvec::prelude::*;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

impl models::table::Table {
    pub fn get_rows(
        &self,
        indices: Option<Vec<u64>>,
    ) -> Result<models::get_rows_result::GetRowsResult, Box<dyn Error>> {
        let result = models::get_rows_result::GetRowsResult::new(
            &indices
                .unwrap_or_else(|| {
                    let table_data_path =
                        utils::get_table_data_path(&self.get_database_id(), &self.get_table_id())
                            .unwrap();

                    let table_data_file = std::fs::OpenOptions::new()
                        .read(true)
                        .open(table_data_path)
                        .unwrap();

                    let row_size = self
                        .get_table_columns()
                        .iter()
                        .fold(0, |acc, column| acc + column.get_size())
                        as u64
                        / 8;

                    let file_size = table_data_file.metadata().unwrap().len();

                    (0..file_size / row_size).collect()
                })
                .into_iter()
                .map(|index| self.get_row(index))
                .collect::<Result<Vec<Vec<u128>>, Box<dyn Error>>>()?,
        )?;

        Ok(result)
    }

    pub(crate) fn get_row(&self, index: u64) -> Result<Vec<u128>, Box<dyn Error>> {
        let table_columns = self.get_table_columns();

        let row_size = table_columns
            .iter()
            .fold(0, |acc, column| acc + column.get_size()) as u64
            / 8;

        let table_data_path =
            utils::get_table_data_path(&self.get_database_id(), &self.get_table_id())?;

        let mut table_data_file = std::fs::OpenOptions::new()
            .read(true)
            .open(table_data_path)?;

        table_data_file.seek(SeekFrom::Start(index * row_size))?;

        let mut buf = vec![0u8; row_size as usize];

        table_data_file.read_exact(&mut buf)?;

        let row_bin: BitVec<u8, Msb0> = BitVec::from_slice(&buf);

        let result = table_columns
            .iter()
            .fold((Vec::new(), 0), |(mut acc, column_start), column| {
                let column_size = column.get_size() as usize;

                let data_bin = &row_bin[column_start..(column_start + column_size)];
                let data_bin = BitVec::from(data_bin);
                let column_start = column_start + column_size;

                let data = data_bin
                    .into_iter()
                    .fold(0, |acc, bit| (acc << 1) + (bit as u128));

                acc.push(data);
                (acc, column_start)
            })
            .0;

        Ok(result)
    }
}
