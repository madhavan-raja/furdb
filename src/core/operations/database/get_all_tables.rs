use crate::core::utils;

use crate::core::models::Database;

use crate::core::error::TableReadError;
use std::io::ErrorKind;

impl Database {
    pub fn get_all_tables(&self) -> Result<Vec<String>, TableReadError> {
        let config = self.get_config();
        let database_info = self.get_database_info();

        let all_tables_path =
            utils::get_all_tables_path(&config.workdir, &database_info.get_database_id());

        let table_ids = std::fs::read_dir(all_tables_path)
            .map_err(|e| match e.kind() {
                ErrorKind::NotFound => TableReadError::NotFound,
                _ => TableReadError::OtherError(e.to_string()),
            })?
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<String>>();

        Ok(table_ids)
    }
}
