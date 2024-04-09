use actix_web::http::StatusCode;
use furdb_core::models as core_models;

use crate::models;

use models::response::success_response::SuccessResponse;
use models::response::success_response::SuccessResponseType;

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

impl Into<SuccessResponse> for ServerHealthResponse {
    fn into(self) -> SuccessResponse {
        SuccessResponse {
            status_code: StatusCode::OK.as_u16(),
            response: SuccessResponseType::ServerHealth(self),
        }
    }
}
