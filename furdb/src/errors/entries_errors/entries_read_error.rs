use actix_web::http::StatusCode;
use furdb_core::errors::entry_errors::entry_read_error::EntryReadError;

use crate::models::response::error_response::ErrorResponse;

impl From<EntryReadError> for ErrorResponse {
    fn from(error: EntryReadError) -> Self {
        match error {
            EntryReadError::InvalidIndex => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error: error.to_string(),
            },
            EntryReadError::InvalidColumn => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error: error.to_string(),
            },
            EntryReadError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: format!("Cannot read data: {e}"),
            },
        }
    }
}
