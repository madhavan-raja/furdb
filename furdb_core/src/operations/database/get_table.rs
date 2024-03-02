use crate::{Database, Table, TableInfo};
use std::error::Error;

impl Database {
    pub fn get_all_table_ids(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut tables = Vec::new();

        for file in std::fs::read_dir(&self.dir)? {
            let file_name = file?;
            let metadata = std::fs::metadata(&file_name.path());

            if metadata?.is_dir() {
                tables.push(file_name.file_name().to_string_lossy().to_string());
            }
        }

        Ok(tables)
    }

    pub fn get_table(
        &self,
        table_id: &str,
        table_info: Option<TableInfo>,
    ) -> Result<Table, Box<dyn Error>> {
        let mut table_dir_path = self.dir.clone();
        table_dir_path.push(table_id);
        let tb = Table::new(table_dir_path, table_info)?;

        Ok(tb)
    }
}
