use furdb_core::Column;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateTableParams {
    pub(crate) table_name: Option<String>,
    pub(crate) table_columns: Vec<Column>,
}
