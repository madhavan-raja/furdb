use crate::core::errors::database_errors::database_creation_error::DatabaseCreationError;
use std::io::ErrorKind;

use crate::core::models;
use crate::core::utils;

impl models::furdb::FurDB {
    pub fn create_database(
        &self,
        database_id: &str,
        database_name: Option<&str>,
    ) -> Result<(), DatabaseCreationError> {
        let config = self.get_config();
        let database_info = &models::database::DatabaseInfo::new(
            &database_id,
            Some(&database_name.unwrap_or(database_id)),
        );

        if !utils::is_id_valid(database_id) {
            return Err(DatabaseCreationError::InvalidId);
        }

        let database_path = utils::get_database_path(&config.fur_directory, database_id);
        let all_tables_path = utils::get_all_tables_path(&config.fur_directory, &database_id);
        let database_config_path =
            utils::get_database_config_path(&config.fur_directory, &database_id);

        std::fs::create_dir(&database_path).map_err(|e| match e.kind() {
            ErrorKind::AlreadyExists => DatabaseCreationError::AlreadyExists,
            _ => DatabaseCreationError::OtherError(e.to_string()),
        })?;

        std::fs::create_dir(&all_tables_path).map_err(|e| match e.kind() {
            ErrorKind::AlreadyExists => DatabaseCreationError::AlreadyExists,
            _ => DatabaseCreationError::OtherError(e.to_string()),
        })?;

        let database_info_serialized = serde_json::to_string(&database_info)
            .map_err(|e| DatabaseCreationError::OtherError(e.to_string()))?;

        std::fs::write(&database_config_path, database_info_serialized).map_err(|e| {
            match e.kind() {
                _ => DatabaseCreationError::OtherError(e.to_string()),
            }
        })?;

        Ok(())
    }
}
