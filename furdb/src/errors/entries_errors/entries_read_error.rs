use actix_web::http::StatusCode;
use furdb_core::errors::query_errors::read_query_error::ReadQueryError;

use crate::models::response::error_response::ErrorResponse;

impl From<ReadQueryError> for ErrorResponse {
    fn from(error: ReadQueryError) -> Self {
        match error {
            ReadQueryError::InvalidIndex => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error: error.to_string(),
            },
            ReadQueryError::InvalidColumn => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error: error.to_string(),
            },
            ReadQueryError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: format!("Cannot read data: {e}"),
            },
        }
    }
}
