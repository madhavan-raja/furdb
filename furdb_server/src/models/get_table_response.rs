use std::error::Error;

use furdb_core::models as core_models;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetTableResponse {
    pub(crate) database_id: String,
    pub(crate) table_id: String,
    pub(crate) table_name: String,
    pub(crate) table_columns: Vec<core_models::column::Column>,
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
