use actix_web::http::StatusCode;
use furdb_core::errors::database_errors::database_creation_error::DatabaseCreationError;

use crate::models::response::error_response::ErrorResponse;

impl From<DatabaseCreationError> for ErrorResponse {
    fn from(error: DatabaseCreationError) -> Self {
        match error {
            DatabaseCreationError::AlreadyExists => ErrorResponse {
                status_code: StatusCode::CONFLICT.as_u16(),
                error: error.to_string(),
            },
            DatabaseCreationError::InvalidId => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error: error.to_string(),
            },
            DatabaseCreationError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: format!("Error creating Database: {e}"),
            },
        }
    }
}
