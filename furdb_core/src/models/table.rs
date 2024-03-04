use crate::models;
use std::error::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Table {
    config: models::config::Config,
    table_info: TableInfo,
}

impl Table {
    pub fn new(
        config: &models::config::Config,
        table_info: &TableInfo,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            config: config.clone(),
            table_info: table_info.clone(),
        })
    }

    pub fn get_config(&self) -> models::config::Config {
        self.config.clone()
    }

    pub fn get_table_info(&self) -> TableInfo {
        self.table_info.clone()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableInfo {
    database_id: String,
    table_id: String,
    table_name: String,
    table_columns: Vec<models::column::Column>,
}

impl TableInfo {
    pub fn new(
        database_id: &str,
        table_id: &str,
        table_name: &str,
        table_columns: &[models::column::Column],
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            database_id: String::from(database_id),
            table_id: String::from(table_id),
            table_name: String::from(table_name),
            table_columns: table_columns.to_vec(),
        })
    }

    pub fn get_database_id(&self) -> String {
        self.database_id.clone()
    }

    pub fn get_table_id(&self) -> String {
        self.table_id.clone()
    }

    pub fn get_table_name(&self) -> String {
        self.table_name.clone()
    }

    pub fn get_table_columns(&self) -> Vec<models::column::Column> {
        self.table_columns.clone()
    }
}
