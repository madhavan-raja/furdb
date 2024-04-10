use actix_web::{http::StatusCode, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use std::fmt::{Display, Formatter};

use super::{error_response::ErrorResponse, success_response::SuccessResponse};

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
    pub fn generate(api_response: &ApiResponse) -> (Self, StatusCode) {
        let status_code = match api_response {
            ApiResponse::Success(response) => match response {
                SuccessResponse::ServerHealth(_) => StatusCode::OK,
                SuccessResponse::DatabaseCreated => StatusCode::CREATED,
                SuccessResponse::DatabaseInfo(_) => StatusCode::OK,
                SuccessResponse::DatabaseDeleted => StatusCode::OK,
                SuccessResponse::TableCreated => StatusCode::CREATED,
                SuccessResponse::TableInfo(_) => StatusCode::OK,
                SuccessResponse::TableDeleted => StatusCode::OK,
                SuccessResponse::EntriesCreated => StatusCode::CREATED,
                SuccessResponse::EntriesResult(_) => StatusCode::OK,
                SuccessResponse::EntriesDeleted => StatusCode::OK,
            },
            ApiResponse::Error(response) => match response {
                ErrorResponse::NotFound(_) => StatusCode::NOT_FOUND,
                ErrorResponse::BadRequest(_) => StatusCode::BAD_REQUEST,
                ErrorResponse::Conflict(_) => StatusCode::CONFLICT,
                ErrorResponse::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };

        let status = status_code.canonical_reason().unwrap_or("").to_string();

        let result = match api_response {
            ApiResponse::Success(_) => "success".to_string(),
            ApiResponse::Error(_) => "error".to_string(),
        };

        (
            Self {
                status_code: status_code.as_u16(),
                status,
                result,
                response: api_response.to_owned(),
            },
            status_code,
        )
    }
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Responder for ApiResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let (response, status_code) = ApiResponseSerializable::generate(&self);
        HttpResponse::build(status_code).json(response)
    }
}

impl ResponseError for ApiResponse {
    fn error_response(&self) -> HttpResponse {
        let (response, status_code) = ApiResponseSerializable::generate(self);
        HttpResponse::build(status_code).json(response)
    }
}
