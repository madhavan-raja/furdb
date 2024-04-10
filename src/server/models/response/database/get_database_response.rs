use serde::{Deserialize, Serialize};

use crate::core::models::column::Column;
use crate::core::models::database::Database;
use crate::core::models::table::Table;

use crate::core::errors::table_errors::table_read_error::TableReadError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDatabaseResponse {
    database_id: String,
    database_name: String,
    database_tables: Vec<GetDatabaseTableResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetDatabaseTableResponse {
    table_id: String,
    table_name: String,
    table_columns: Vec<Column>,
}

impl GetDatabaseResponse {
    pub fn new(database: &Database) -> Result<Self, TableReadError> {
        let database_info = database.get_database_info();

        Ok(Self {
            database_id: database_info.get_database_id(),
            database_name: database_info.get_database_name(),
            database_tables: database
                .get_all_tables()?
                .into_iter()
                .map(|table| GetDatabaseTableResponse::new(&table))
                .collect(),
        })
    }
}

impl GetDatabaseTableResponse {
    pub fn new(table: &Table) -> Self {
        let table_info = table.get_table_info();

        Self {
            table_id: table_info.get_table_id(),
            table_name: table_info.get_table_name(),
            table_columns: table_info.get_table_columns(),
        }
    }
}
