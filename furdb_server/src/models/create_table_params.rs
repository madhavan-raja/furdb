use furdb_core::models as core_models;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateTableParams {
    pub(crate) table_name: Option<String>,
    pub(crate) table_columns: Vec<core_models::column::Column>,
}
