use furdb_core::Column;

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct CreateTableParams {
    pub table_name: Option<String>,
    pub table_columns: Vec<Column>,
}
