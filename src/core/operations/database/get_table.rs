use crate::core::errors::table_errors::table_read_error::TableReadError;
use std::io::ErrorKind;

use crate::core::models;
use crate::core::utils;

impl models::database::Database {
    pub fn get_table(&self, table_id: &str) -> Result<models::table::Table, TableReadError> {
        let config = self.get_config();
        let database_info = self.get_database_info();

        let table_config_path = utils::get_table_config_path(
            &config.fur_directory,
            &database_info.get_database_id(),
            table_id,
        );

        let table_info_file =
            std::fs::File::open(&table_config_path).map_err(|e| match e.kind() {
                ErrorKind::NotFound => TableReadError::NotFound,
                _ => TableReadError::OtherError(e.to_string()),
            })?;

        let table_info = serde_json::from_reader(table_info_file)
            .map_err(|e| TableReadError::OtherError(e.to_string()))?;

        let table = models::table::Table::new(&config, &table_info);

        Ok(table)
    }
}
