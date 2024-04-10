use furdb_core::models as core_models;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerHealthResponse {
    message: String,
    config: core_models::config::Config,
}

impl ServerHealthResponse {
    pub fn new(config: &core_models::config::Config) -> Self {
        Self {
            message: String::from("Server is running"),
            config: config.to_owned(),
        }
    }
}
