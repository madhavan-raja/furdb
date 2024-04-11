use serde::{Deserialize, Serialize};

use crate::core::furdb_config::FurDBConfig;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfoResponse {
    message: String,
    config: FurDBConfig,
}

impl ServerInfoResponse {
    pub fn new(config: &FurDBConfig) -> Self {
        Self {
            message: String::from("Server is running"),
            config: config.to_owned(),
        }
    }
}
