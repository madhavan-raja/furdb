use serde::{Deserialize, Serialize};

use crate::core::models::column::Column;
use crate::core::models::table::Table;

#[derive(Serialize, Deserialize, Clone)]
pub struct GetTableResponse {
    database_id: String,
    table_id: String,
    table_name: String,
    table_columns: Vec<Column>,
}

impl GetTableResponse {
    pub fn new(table: &Table) -> Self {
        let table_info = table.get_table_info();

        Self {
            database_id: table_info.get_database_id(),
            table_id: table_info.get_table_id(),
            table_name: table_info.get_table_name(),
            table_columns: table_info.get_table_columns(),
        }
    }
}
