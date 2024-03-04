use crate::models;
use crate::utils;
use std::error::Error;

impl models::furdb::FurDB {
    pub fn create_database(
        &self,
        database_id: &str,
        database_name: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let config = self.get_config();
        let database_info = &models::database::DatabaseInfo::new(
            &database_id,
            Some(&database_name.unwrap_or(database_id)),
        )?;

        let database_path = utils::get_database_path(&config.fur_directory, database_id)?;
        let all_tables_path = utils::get_all_tables_path(&config.fur_directory, &database_id)?;
        let database_config_path =
            utils::get_database_config_path(&config.fur_directory, &database_id)?;

        std::fs::create_dir(&database_path)?;
        std::fs::create_dir(&all_tables_path)?;
        std::fs::write(
            &database_config_path,
            serde_json::to_string(&database_info)?,
        )?;

        Ok(())
    }
}
