use std::io::ErrorKind;
use crate::errors::table_errors::table_deletion_error::TableDeletionError;

use crate::models;
use crate::utils;

impl models::database::Database {
    pub fn delete_all_tables(&self) -> Result<(), TableDeletionError> {
        let config = self.get_config();
        let database_info = self.get_database_info();

        let all_tables_path =
            utils::get_all_tables_path(&config.fur_directory, &database_info.get_database_id());

        let table_ids = std::fs::read_dir(all_tables_path)
            .map_err(|e| match e.kind() {
                ErrorKind::NotFound => TableDeletionError::NotFound,
                _ => TableDeletionError::OtherError(e.to_string()),
            })?
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<String>>();

        table_ids
            .iter()
            .map(|table_id| self.delete_table(table_id))
            .collect::<Result<(), TableDeletionError>>()?;

        Ok(())
    }
}
