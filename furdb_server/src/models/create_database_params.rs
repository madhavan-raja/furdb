#[derive(serde::Deserialize)]
pub struct CreateDatabaseParams {
    pub(crate) db_name: Option<String>,
}
