use actix_web::http::StatusCode;
use serde::Serialize;

use crate::server::models::response::error_response::ErrorResponse;
use crate::server::models::response::success_response::SuccessResponse;

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum ApiResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

#[derive(Debug, Serialize)]
pub struct ApiResponseSerializable {
    pub result: String,
    pub status_code: u16,
    pub status: String,
    pub response: ApiResponse,
}

impl ApiResponseSerializable {
    pub fn generate_success(response: &SuccessResponse) -> (Self, StatusCode) {
        let status_code = match response {
            SuccessResponse::ServerInfo(_) => StatusCode::OK,
            SuccessResponse::DatabaseCreated => StatusCode::CREATED,
            SuccessResponse::DatabaseInfo(_) => StatusCode::OK,
            SuccessResponse::DatabaseDeleted => StatusCode::OK,
            SuccessResponse::TableCreated => StatusCode::CREATED,
            SuccessResponse::TableInfo(_) => StatusCode::OK,
            SuccessResponse::TableDeleted => StatusCode::OK,
            SuccessResponse::EntriesCreated => StatusCode::CREATED,
            SuccessResponse::EntriesResult(_) => StatusCode::OK,
            SuccessResponse::EntriesDeleted => StatusCode::OK,
        };

        let status = status_code.canonical_reason().unwrap_or("").to_string();

        (
            Self {
                status_code: status_code.as_u16(),
                status,
                result: "success".to_string(),
                response: ApiResponse::Success(response.to_owned()),
            },
            status_code,
        )
    }

    pub fn generate_error(response: &ErrorResponse) -> (Self, StatusCode) {
        let status_code = match response {
            ErrorResponse::NotFound(_) => StatusCode::NOT_FOUND,
            ErrorResponse::BadRequest(_) => StatusCode::BAD_REQUEST,
            ErrorResponse::Conflict(_) => StatusCode::CONFLICT,
            ErrorResponse::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let status = status_code.canonical_reason().unwrap_or("").to_string();

        (
            Self {
                status_code: status_code.as_u16(),
                status,
                result: "error".to_string(),
                response: ApiResponse::Error(response.to_owned()),
            },
            status_code,
        )
    }
}
