use std::error::Error;

use furdb_core::models as core_models;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct ServerHealthResponse {
    message: String,
    config: core_models::config::Config,
}

impl ServerHealthResponse {
    pub(crate) fn new(config: &core_models::config::Config) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            message: String::from("Server is running"),
            config: config.to_owned(),
        })
    }
}
