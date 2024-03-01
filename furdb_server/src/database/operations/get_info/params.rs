#[derive(serde::Deserialize)]
pub(crate) struct CreateDatabaseParams {
    pub db_name: String,
}

#[derive(serde::Deserialize)]
pub(crate) struct GetDatabaseParams {}
