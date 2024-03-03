use std::error::Error;

use furdb_core::models as core_models;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetTableResponse {
    database_id: String,
    table_id: String,
    table_name: String,
    table_columns: Vec<core_models::column::Column>,
}

impl GetTableResponse {
    pub(crate) fn new(table: &core_models::table::Table) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            database_id: table.get_database_id(),
            table_id: table.get_table_id(),
            table_name: table.get_table_name(),
            table_columns: table.get_table_columns(),
        })
    }
}
