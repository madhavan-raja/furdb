use crate::core::models;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Database {
    config: models::config::Config,
    database_info: DatabaseInfo,
}

impl Database {
    pub fn new(config: &models::config::Config, database_info: &DatabaseInfo) -> Self {
        Self {
            config: config.to_owned(),
            database_info: database_info.to_owned(),
        }
    }

    pub fn get_config(&self) -> models::config::Config {
        self.config.to_owned()
    }

    pub fn get_database_info(&self) -> DatabaseInfo {
        self.database_info.to_owned()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseInfo {
    database_id: String,
    database_name: String,
}

impl DatabaseInfo {
    pub fn new(database_id: &str, database_name: Option<&str>) -> Self {
        Self {
            database_id: database_id.to_string(),
            database_name: database_name.unwrap_or(database_id).to_string(),
        }
    }

    pub fn get_database_id(&self) -> String {
        self.database_id.to_owned()
    }

    pub fn get_database_name(&self) -> String {
        self.database_name.to_owned()
    }
}
