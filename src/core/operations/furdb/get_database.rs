use crate::core::utils;

use crate::core::furdb::FurDB;
use crate::core::models::database::Database;

use crate::core::error::DatabaseReadError;
use std::io::ErrorKind;

impl FurDB {
    pub fn get_database(&self, database_id: &str) -> Result<Database, DatabaseReadError> {
        let config = self.get_config();

        let database_config_path = utils::get_database_config_path(&config.workdir, database_id);

        let database_config_file =
            std::fs::File::open(database_config_path).map_err(|e| match e.kind() {
                ErrorKind::NotFound => DatabaseReadError::NotFound,
                _ => DatabaseReadError::OtherError(e.to_string()),
            })?;

        let database_info = serde_json::from_reader(database_config_file)
            .map_err(|e| DatabaseReadError::OtherError(e.to_string()))?;

        let database = Database::new(&config, &database_info);

        Ok(database)
    }
}
