use crate::FurTableInfo;
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct FurTable {
    dir: PathBuf,
    data_file_size: u64,
    table_info: FurTableInfo,
}

mod operations;
mod utils;

impl FurTable {
    pub fn new(dir: PathBuf, table_info: Option<FurTableInfo>) -> Result<Self, Box<dyn Error>> {
        Self::ensure_table_files(&dir)?;

        let data_file_size = Self::get_data_file_size(&dir)?;

        let table_info = if table_info.is_some() {
            table_info.unwrap()
        } else {
            Self::load_info(&dir)?
        };

        Ok(Self {
            dir,
            data_file_size,
            table_info,
        })
    }

    pub fn get_info(&self) -> Result<&FurTableInfo, Box<dyn Error>> {
        Ok(&self.table_info)
    }

    pub fn save_info(&self) -> Result<(), Box<dyn Error>> {
        let table_info_raw = serde_json::to_string(&self.table_info)?;
        let table_info_file_path = Self::get_info_file_path(&self.dir);
        std::fs::write(table_info_file_path, table_info_raw)?;

        Ok(())
    }
}
