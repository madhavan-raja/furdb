use serde::{Deserialize, Serialize};

use crate::core::furdb_config::FurDBConfig;
use crate::core::models::column::Column;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    config: FurDBConfig,
    table_info: TableInfo,
}

impl Table {
    pub fn new(config: &FurDBConfig, table_info: &TableInfo) -> Self {
        Self {
            config: config.to_owned(),
            table_info: table_info.to_owned(),
        }
    }

    pub fn get_config(&self) -> FurDBConfig {
        self.config.to_owned()
    }

    pub fn get_table_info(&self) -> TableInfo {
        self.table_info.to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    database_id: String,
    table_id: String,
    table_name: String,
    table_columns: Vec<Column>,
}

impl TableInfo {
    pub fn new(
        database_id: &str,
        table_id: &str,
        table_name: &str,
        table_columns: &[Column],
    ) -> Self {
        Self {
            database_id: database_id.to_string(),
            table_id: table_id.to_string(),
            table_name: table_name.to_string(),
            table_columns: table_columns.to_vec(),
        }
    }

    pub fn get_database_id(&self) -> String {
        self.database_id.to_owned()
    }

    pub fn get_table_id(&self) -> String {
        self.table_id.to_owned()
    }

    pub fn get_table_name(&self) -> String {
        self.table_name.to_owned()
    }

    pub fn get_table_columns(&self) -> Vec<Column> {
        self.table_columns.to_owned()
    }
}
