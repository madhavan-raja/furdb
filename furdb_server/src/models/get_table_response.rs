use furdb_core::Column;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetTableResponse {
    pub(crate) table_id: String,
    pub(crate) table_name: String,
    pub(crate) table_columns: Vec<Column>,
}

impl GetTableResponse {
    pub fn new(table_id: &str, table_name: &str, table_columns: Vec<Column>) -> Self {
        Self {
            table_id: String::from(table_id),
            table_name: String::from(table_name),
            table_columns,
        }
    }
}
