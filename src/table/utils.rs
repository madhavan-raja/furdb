use crate::{FurTable, FurTableInfo};
use bitvec::prelude::*;
use std::{collections::HashMap, error::Error, io::Write, path::PathBuf};

impl FurTable {
    pub(super) fn ensure_table_files(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        if !dir.exists() {
            std::fs::create_dir(&dir)?;
        }

        Self::ensure_data_file(dir)?;

        Ok(())
    }

    pub(super) fn ensure_data_file(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        let data_file_path = Self::get_data_file_path(&dir);
        if !data_file_path.exists() {
            std::fs::write(data_file_path, "")?;
        }

        Ok(())
    }

    pub(super) fn load_info(dir: &PathBuf) -> Result<FurTableInfo, Box<dyn Error>> {
        let table_info_file_path = Self::get_info_file_path(&dir);
        let table_info_contents_raw = std::fs::read_to_string(&table_info_file_path)?;
        let table_info_contents = serde_json::from_str(&table_info_contents_raw)?;
        let table_info = serde_json::from_value(table_info_contents)?;

        Ok(table_info)
    }

    pub(super) fn save_info(&self) -> Result<(), Box<dyn Error>> {
        let table_info_raw = serde_json::to_string(&self.table_info)?;
        let table_info_file_path = Self::get_info_file_path(&self.dir);
        std::fs::write(table_info_file_path, table_info_raw)?;

        Ok(())
    }

    pub(super) fn get_data_file_size(dir: &PathBuf) -> Result<u64, Box<dyn Error>> {
        let data_file_metadata = std::fs::metadata(Self::get_data_file_path(&dir))?;
        let data_file_size = data_file_metadata.len();
        Ok(data_file_size)
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

    pub(super) fn get_row_size(&mut self) -> Result<usize, Box<dyn Error>> {
        let table_info = self.get_info()?;
        let mut size = 0;

        for column in table_info.get_columns() {
            size += column.get_size();
        }

        Ok(size as usize)
    }

    pub(super) fn write_data(&mut self, bytes: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.data_file.write(&bytes)?;

        Ok(())
    }

    pub(super) fn get_info_file_path(dir: &PathBuf) -> PathBuf {
        let mut table_info_file_path = dir.clone();
        table_info_file_path.push("fur_table.json");

        table_info_file_path
    }

    pub(super) fn get_data_file_path(dir: &PathBuf) -> PathBuf {
        let mut data_file_path = dir.clone();
        data_file_path.push("data.fur");

        data_file_path
    }
}
