use std::error::Error;

use furdb_core::{Column, Database, Table};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetDatabaseResponse {
    pub(crate) database_id: String,
    pub(crate) database_name: String,
    pub(crate) database_tables: Vec<GetDatabaseTableResponse>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetDatabaseTableResponse {
    pub(crate) table_id: String,
    pub(crate) table_name: String,
    pub(crate) table_columns: Vec<Column>,
}

impl GetDatabaseResponse {
    pub(crate) fn new(database: &Database) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            database_id: database.get_database_id(),
            database_name: database.get_database_name(),
            database_tables: database
                .get_all_tables()?
                .into_iter()
                .map(|table| GetDatabaseTableResponse::new(&table).unwrap())
                .collect(),
        })
    }
}

impl GetDatabaseTableResponse {
    pub(crate) fn new(table: &Table) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            table_id: table.get_table_id(),
            table_name: table.get_table_name(),
            table_columns: table.get_table_columns(),
        })
    }
}
