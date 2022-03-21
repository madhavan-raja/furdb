use crate::FurTable;
use std::error::Error;

impl FurTable {
    pub fn delete_all_rows(&self) -> Result<(), Box<dyn Error>> {
        let data_file_path = Self::get_data_file_path(&self.dir);
        std::fs::write(data_file_path, "")?;

        Ok(())
    }
}
