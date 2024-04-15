use serde::{Deserialize, Serialize};

use crate::core::models::Column;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTableParams {
    table_columns: Vec<Column>,
}

impl CreateTableParams {
    pub fn get_table_columns(&self) -> Vec<Column> {
        self.table_columns.to_vec()
    }
}
