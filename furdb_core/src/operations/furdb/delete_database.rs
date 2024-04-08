use crate::errors::database_errors::database_deletion_error::DatabaseDeletionError;
use crate::models;
use crate::utils;

impl models::furdb::FurDB {
    pub fn delete_database(&self, database_id: &str) -> Result<(), DatabaseDeletionError> {
        let config = self.get_config();

        let database_path = utils::get_database_path(&config.fur_directory, database_id);

        std::fs::remove_dir_all(&database_path).map_err(|e| {
            match e.kind() {
                std::io::ErrorKind::NotFound => DatabaseDeletionError::NotFound,
                _ => DatabaseDeletionError::OtherError(e.to_string()),
            }
        })?;

        Ok(())
    }
}
