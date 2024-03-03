use crate::models;
use crate::utils;
use std::error::Error;

impl models::database::Database {
    pub fn create_database(
        database_id: &str,
        database_name: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let database_path = utils::get_database_path(database_id)?;
        let all_tables_path = utils::get_all_tables_path(database_id)?;
        let database_config_path = utils::get_database_config_path(database_id)?;

        std::fs::create_dir(&database_path)?;
        std::fs::create_dir(&all_tables_path)?;

        let database =
            models::database::Database::new(&database_id, &database_name.unwrap_or(database_id))?;

        std::fs::write(&database_config_path, serde_json::to_string(&database)?)?;

        Ok(())
    }
}
