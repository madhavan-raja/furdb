#[derive(serde::Deserialize)]
pub struct CreateDatabaseParams {
    pub(crate) database_name: Option<String>,
}
