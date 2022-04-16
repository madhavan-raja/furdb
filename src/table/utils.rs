use crate::{FurTable, FurTableInfo};
use std::{error::Error, path::PathBuf};

impl FurTable {
    pub(crate) fn ensure_table_files(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        if !dir.exists() {
            std::fs::create_dir(&dir)?;
        }

        Self::ensure_data_file(&dir)?;

        Self::ensure_sortfile_directory(&dir)?;

        Ok(())
    }

    pub(crate) fn ensure_data_file(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        let data_file_path = Self::get_data_file_path(&dir);
        if !data_file_path.exists() {
            std::fs::write(data_file_path, "")?;
        }

        Ok(())
    }

    pub(crate) fn ensure_sortfile_directory(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        let sortfile_directory_path = Self::get_sortfile_directory(&dir);

        if !sortfile_directory_path.exists() {
            std::fs::create_dir(&sortfile_directory_path)?;
        }

        Ok(())
    }

    pub(crate) fn load_info(dir: &PathBuf) -> Result<FurTableInfo, Box<dyn Error>> {
        let table_info_file_path = Self::get_info_file_path(&dir);
        let table_info_contents_raw = std::fs::read_to_string(&table_info_file_path)?;
        let table_info_contents = serde_json::from_str(&table_info_contents_raw)?;
        let table_info = serde_json::from_value(table_info_contents)?;

        Ok(table_info)
    }

    pub(crate) fn get_data_file_size(dir: &PathBuf) -> Result<u64, Box<dyn Error>> {
        let data_file_metadata = std::fs::metadata(Self::get_data_file_path(&dir))?;
        let data_file_size = data_file_metadata.len();
        Ok(data_file_size)
    }

    pub(crate) fn get_row_size(&self) -> Result<usize, Box<dyn Error>> {
        let table_info = self.get_info()?;
        let mut size = 0;

        for column in table_info.get_columns() {
            size += column.get_size();
        }

        Ok(size as usize)
    }

    pub(crate) fn get_info_file_path(dir: &PathBuf) -> PathBuf {
        let mut table_info_file_path = dir.clone();
        table_info_file_path.push("fur_table.json");

        table_info_file_path
    }

    pub(crate) fn get_data_file_path(dir: &PathBuf) -> PathBuf {
        let mut data_file_path = dir.clone();
        data_file_path.push("data.fur");

        data_file_path
    }

    pub(crate) fn get_sortfile_directory(dir: &PathBuf) -> PathBuf {
        let mut sortfile_directory = dir.clone();
        sortfile_directory.push("sortfiles");

        sortfile_directory
    }

    pub(crate) fn get_sortfile_path(dir: &PathBuf, column_id: &str) -> PathBuf {
        let mut sortfile_path = Self::get_sortfile_directory(&dir);
        sortfile_path.push(format!("{column_id}.sortfile"));

        sortfile_path
    }
}
