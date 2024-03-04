use crate::models;
use std::error::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Database {
    config: models::config::Config,
    database_info: DatabaseInfo,
}

impl Database {
    pub fn new(
        config: &models::config::Config,
        database_info: &DatabaseInfo,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            config: config.clone(),
            database_info: database_info.clone(),
        })
    }

    pub fn get_config(&self) -> models::config::Config {
        self.config.clone()
    }

    pub fn get_database_info(&self) -> DatabaseInfo {
        self.database_info.clone()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseInfo {
    database_id: String,
    database_name: String,
}

impl DatabaseInfo {
    pub fn new(database_id: &str, database_name: Option<&str>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            database_id: String::from(database_id),
            database_name: String::from(database_name.unwrap_or(database_id)),
        })
    }

    pub fn get_database_id(&self) -> String {
        self.database_id.clone()
    }

    pub fn get_database_name(&self) -> String {
        self.database_name.clone()
    }
}
