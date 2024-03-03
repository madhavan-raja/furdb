use furdb_core::models as core_models;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct CreateTableParams {
    table_name: Option<String>,
    table_columns: Vec<core_models::column::Column>,
}

impl CreateTableParams {
    pub(crate) fn get_table_name(&self) -> Option<String> {
        self.table_name.clone()
    }

    pub(crate) fn get_table_columns(&self) -> Vec<core_models::column::Column> {
        self.table_columns.to_vec()
    }
}
