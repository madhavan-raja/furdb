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
        let table_info = table.get_table_info();

        Ok(Self {
            database_id: table_info.get_database_id(),
            table_id: table_info.get_table_id(),
            table_name: table_info.get_table_name(),
            table_columns: table_info.get_table_columns(),
        })
    }
}
