use serde::{Deserialize, Serialize};

use furdb_core::models::column::Column;

#[derive(Serialize, Deserialize)]
pub struct CreateTableParams {
    table_name: Option<String>,
    table_columns: Vec<Column>,
}

impl CreateTableParams {
    pub fn get_table_name(&self) -> Option<String> {
        self.table_name.to_owned()
    }

    pub fn get_table_columns(&self) -> Vec<Column> {
        self.table_columns.to_vec()
    }
}
