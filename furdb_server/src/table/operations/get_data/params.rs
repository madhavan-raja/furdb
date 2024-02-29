#[derive(serde::Deserialize)]
pub(crate) struct TableParams {
    pub db_name: Option<String>,
    pub working_dir: Option<String>,
}
