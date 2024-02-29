#[derive(serde::Deserialize)]
pub(crate) struct DatabaseParams {
    pub db_name: Option<String>,
    pub working_dir: Option<String>,
}
