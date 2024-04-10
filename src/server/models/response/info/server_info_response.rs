use serde::{Deserialize, Serialize};

use crate::core::models::config::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerInfoResponse {
    message: String,
    config: Config,
}

impl ServerInfoResponse {
    pub fn new(config: &Config) -> Self {
        Self {
            message: String::from("Server is running"),
            config: config.to_owned(),
        }
    }
}
