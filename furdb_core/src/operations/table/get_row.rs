use crate::{Column, Table};
use bitvec::prelude::*;
use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Seek, SeekFrom},
};

impl Table {
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

    pub async fn get_row(&mut self, index: u64) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let row_bin = self.get_row_bin(index)?;
        let mut result = HashMap::<String, String>::new();

        for column in self.table_info.get_columns() {
            let data_type = column.get_data_type();

            let data_bin = row_bin.get(&column.get_id()).unwrap();
            let data = data_type
                .decode(data_bin, self.table_info.get_converter_server())
                .await?;

            result.insert(column.get_id(), data);
        }

        Ok(result)
    }

    pub async fn get_rows(
        &mut self,
        indices: Vec<u64>,
    ) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
        let mut results = Vec::<HashMap<String, String>>::new();

        for index in indices {
            let result = self.get_row(index).await?;

            results.push(result);
        }

        Ok(results)
    }

    pub async fn get_all(&mut self) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
        let row_size = self.get_row_size()? / 8;
        let indices: Vec<u64> =
            (0..Self::get_data_file_size(&self.dir)? / row_size as u64).collect();

        let results = self.get_rows(indices).await?;

        Ok(results)
    }

    pub async fn query(
        &mut self,
        column: &Column,
        value: &str,
    ) -> Result<Option<u64>, Box<dyn Error>> {
        let sortlist = self.read_sortfile(&column.get_id())?.get_sortlist();

        let data_type = column.get_data_type();
        let converter_server = self.table_info.get_converter_server();

        let target_value = data_type
            .encode(value, column.get_size(), converter_server.clone())
            .await?;

        let mut left = 0 as i64;
        let mut right = sortlist.len() as i64 - 1;

        let column_id = column.get_id();

        while left <= right {
            let mid = (left + right) / 2;
            let mid_index = sortlist[mid as usize];
            let mid_row_bin = self.get_row_bin(mid_index as u64)?;
            let mid_value = mid_row_bin.get(&column_id).unwrap().clone();

            match data_type
                .compare(&mid_value, &target_value, converter_server.clone())
                .await?
            {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid - 1,
                std::cmp::Ordering::Equal => return Ok(Some(mid_index as u64)),
            }
        }

        Ok(None)
    }
}
