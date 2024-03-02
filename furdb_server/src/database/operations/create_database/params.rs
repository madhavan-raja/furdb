#[derive(serde::Deserialize)]
pub(crate) struct CreateDatabaseParams {
    pub db_name: Option<String>,
}
