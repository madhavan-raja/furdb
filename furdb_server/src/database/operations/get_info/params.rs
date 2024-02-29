#[derive(serde::Deserialize)]
pub(crate) struct DatabaseParams {
    pub db_name: Option<String>,
}
