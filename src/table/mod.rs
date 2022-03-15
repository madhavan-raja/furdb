use crate::FurTableInfo;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    path::PathBuf,
};

#[derive(Debug)]
pub struct FurTable {
    dir: PathBuf,
    data_file: File,
    data_file_size: u64,
    table_info: FurTableInfo,
}

mod operations;
mod utils;

impl FurTable {
    pub fn new(dir: PathBuf, table_info: Option<FurTableInfo>) -> Result<FurTable, Box<dyn Error>> {
        Self::ensure_table_files(&dir)?;

        let data_file_path = Self::get_data_file_path(&dir);
        let data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(data_file_path)?;

        let data_file_size = Self::get_data_file_size(&dir)?;

        let table_info = table_info.unwrap_or(Self::load_info(&dir)?);

        Ok(FurTable {
            dir,
            data_file,
            data_file_size,
            table_info,
        })
    }

    pub fn get_info(&mut self) -> std::io::Result<&FurTableInfo> {
        Ok(&self.table_info)
    }
}