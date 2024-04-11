use crate::core::utils;

use crate::core::furdb::FurDB;

use crate::core::error::DatabaseDeletionError;
use std::io::ErrorKind;

impl FurDB {
    pub fn delete_database(&self, database_id: &str) -> Result<(), DatabaseDeletionError> {
        let config = self.get_config();

        let database_path = utils::get_database_path(&config.workdir, database_id);

        std::fs::remove_dir_all(&database_path).map_err(|e| match e.kind() {
            ErrorKind::NotFound => DatabaseDeletionError::NotFound,
            _ => DatabaseDeletionError::OtherError(e.to_string()),
        })?;

        Ok(())
    }
}
