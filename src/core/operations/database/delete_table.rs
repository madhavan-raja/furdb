use crate::core::utils;

use crate::core::models::database::Database;

use crate::core::errors::table_errors::table_deletion_error::TableDeletionError;
use std::io::ErrorKind;

impl Database {
    pub fn delete_table(&self, table_id: &str) -> Result<(), TableDeletionError> {
        let config = self.get_config();
        let database_info = self.get_database_info();

        let table_path = utils::get_table_path(
            &config.fur_directory,
            &database_info.get_database_id(),
            table_id,
        );

        std::fs::remove_dir_all(&table_path).map_err(|e| match e.kind() {
            ErrorKind::NotFound => TableDeletionError::NotFound,
            _ => TableDeletionError::OtherError(e.to_string()),
        })?;

        Ok(())
    }
}
