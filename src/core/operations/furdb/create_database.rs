use crate::core::utils;

use crate::core::furdb::FurDB;
use crate::core::models::database::{Database, DatabaseInfo};

use crate::core::error::DatabaseCreationError;
use std::io::ErrorKind;

impl FurDB {
    pub fn create_database(&self, database_id: &str) -> Result<Database, DatabaseCreationError> {
        let config = self.get_config();

        if !utils::is_id_valid(database_id) {
            return Err(DatabaseCreationError::InvalidId);
        }

        let database_path = utils::get_database_path(&config.workdir, database_id);
        let all_tables_path = utils::get_all_tables_path(&config.workdir, database_id);
        let database_config_path = utils::get_database_config_path(&config.workdir, database_id);

        std::fs::create_dir(database_path).map_err(|e| match e.kind() {
            ErrorKind::AlreadyExists => DatabaseCreationError::AlreadyExists,
            _ => DatabaseCreationError::OtherError(e.to_string()),
        })?;

        std::fs::create_dir(all_tables_path).map_err(|e| match e.kind() {
            ErrorKind::AlreadyExists => DatabaseCreationError::AlreadyExists,
            _ => DatabaseCreationError::OtherError(e.to_string()),
        })?;

        let database_info = DatabaseInfo::new(database_id);
        let database = Database::new(&config, &database_info);

        let database_info_serialized = serde_json::to_string(&database_info)
            .map_err(|e| DatabaseCreationError::OtherError(e.to_string()))?;

        std::fs::write(database_config_path, database_info_serialized)
            .map_err(|e| DatabaseCreationError::OtherError(e.to_string()))?;

        Ok(database)
    }
}
