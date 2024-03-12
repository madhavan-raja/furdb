use crate::models;
use crate::utils;
use bitvec::prelude::*;
use std::cmp::{max, min};
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

impl models::table::Table {
    pub fn query(
        &self,
        column_index: u64,
        value: u128,
    ) -> Result<models::query_result::QueryResult, Box<dyn Error>> {
        let config = self.get_config();
        let table_info = self.get_table_info();

        let table_columns = table_info.get_table_columns();
        let row_size = table_columns
            .iter()
            .fold(0, |acc, column| acc + column.get_size()) as u64
            / 8;

        let table_data_path = utils::get_table_data_path(
            &config.fur_directory,
            &table_info.get_database_id(),
            &table_info.get_table_id(),
        )?;

        let table_data_file = std::fs::OpenOptions::new()
            .read(true)
            .open(table_data_path)?;

        let table_data_file_size = table_data_file.metadata()?.len();

        let row_count = table_data_file_size / row_size;

        let identifier_size = if row_count > 0 {
            1 + ((row_count - 1) / 256)
        } else {
            0
        };

        let table_sortfile_path = utils::get_sortfile_path(
            &config.fur_directory,
            &table_info.get_database_id(),
            &table_info.get_table_id(),
        )?;

        let mut table_sortfile = std::fs::OpenOptions::new()
            .read(true)
            .open(table_sortfile_path)?;

        let mut start = row_count - 1;
        let mut end = 0;

        // Start

        let mut left = column_index * row_count * identifier_size;
        let mut right = (column_index + 1) * (row_count - 1) * identifier_size;

        while left <= right {
            let mid = left + (right - left) / 2;

            println!("LEFT: {}, RIGHT: {}, MID: {}", left, right, mid);

            let mut index_bin = vec![0u8; identifier_size as usize];
            table_sortfile.seek(SeekFrom::Start(mid))?;
            table_sortfile.read_exact(&mut index_bin)?;
            let index_bin = BitVec::<u8, Msb0>::from_slice(&index_bin);

            let index = index_bin
                .into_iter()
                .fold(0, |acc, bit| (acc << 1) + (bit as u64));

            let current_value = self.get_row(index)?[column_index as usize];

            if current_value >= value {
                if current_value == value {
                    start = min(start, mid);
                }
                if mid < identifier_size {
                    break;
                }
                right = mid - identifier_size;
            } else {
                left = mid + identifier_size;
            }
        }

        // End

        let mut left = column_index * row_count * identifier_size;
        let mut right = (column_index + 1) * (row_count - 1) * identifier_size;

        while left <= right {
            let mid = left + (right - left) / 2;

            println!("LEFT: {}, RIGHT: {}, MID: {}", left, right, mid);

            let mut index_bin = vec![0u8; identifier_size as usize];
            table_sortfile.seek(SeekFrom::Start(mid))?;
            table_sortfile.read_exact(&mut index_bin)?;
            let index_bin = BitVec::<u8, Msb0>::from_slice(&index_bin);

            let index = index_bin
                .into_iter()
                .fold(0, |acc, bit| (acc << 1) + (bit as u64));

            let current_value = self.get_row(index)?[column_index as usize];

            if current_value <= value {
                if current_value == value {
                    end = max(end, mid);
                }
                left = mid + identifier_size;
            } else {
                if mid < identifier_size {
                    break;
                }
                right = mid - identifier_size;
            }
        }

        println!("START: {}, END: {}", start, end);

        let indices = (start..(end + identifier_size))
            .map(|offset| {
                let mut index_bin = vec![0u8; identifier_size as usize];
                table_sortfile.seek(SeekFrom::Start(offset)).unwrap();
                table_sortfile.read_exact(&mut index_bin).unwrap();
                let index_bin = BitVec::<u8, Msb0>::from_slice(&index_bin);

                let index = index_bin
                    .into_iter()
                    .fold(0, |acc, bit| (acc << 1) + (bit as u64));

                index
            })
            .collect();

        self.get_rows(Some(indices))
    }

    pub fn get_rows(
        &self,
        indices: Option<Vec<u64>>,
    ) -> Result<models::query_result::QueryResult, Box<dyn Error>> {
        let config = self.get_config();
        let table_info = self.get_table_info();

        let table_data_path = utils::get_table_data_path(
            &config.fur_directory,
            &table_info.get_database_id(),
            &table_info.get_table_id(),
        )?;

        let result = models::query_result::QueryResult::new(
            &indices
                .unwrap_or_else(|| {
                    let table_data_file = std::fs::OpenOptions::new()
                        .read(true)
                        .open(table_data_path)
                        .unwrap();

                    let row_size = table_info
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
        let config = self.get_config();
        let table_info = self.get_table_info();

        let table_columns = table_info.get_table_columns();

        let row_size = table_columns
            .iter()
            .fold(0, |acc, column| acc + column.get_size()) as u64
            / 8;

        let table_data_path = utils::get_table_data_path(
            &config.fur_directory,
            &table_info.get_database_id(),
            &table_info.get_table_id(),
        )?;

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
