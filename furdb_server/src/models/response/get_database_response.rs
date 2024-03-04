use std::error::Error;

use furdb_core::models as core_models;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetDatabaseResponse {
    database_id: String,
    database_name: String,
    database_tables: Vec<GetDatabaseTableResponse>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct GetDatabaseTableResponse {
    table_id: String,
    table_name: String,
    table_columns: Vec<core_models::column::Column>,
}

impl GetDatabaseResponse {
    pub(crate) fn new(database: &core_models::database::Database) -> Result<Self, Box<dyn Error>> {
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
    pub(crate) fn new(table: &core_models::table::Table) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            table_id: table.get_table_id(),
            table_name: table.get_table_name(),
            table_columns: table.get_table_columns(),
        })
    }
}
