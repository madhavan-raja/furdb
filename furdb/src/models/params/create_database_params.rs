#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct CreateDatabaseParams {
    database_name: Option<String>,
}

impl CreateDatabaseParams {
    pub(crate) fn get_database_name(&self) -> Option<String> {
        self.database_name.clone()
    }
}
