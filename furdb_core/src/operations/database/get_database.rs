use crate::{utils, Database};
use std::error::Error;

impl Database {
    pub fn get_database(database_id: &str) -> Result<Self, Box<dyn Error>> {
        let database_config_path = utils::get_database_config_path(database_id)?;
        let database = serde_json::from_reader(std::fs::File::open(&database_config_path)?)?;

        Ok(database)
    }
}
