use actix_web::http::StatusCode;
use furdb_core::errors::database_errors::database_creation_error::DatabaseCreationError;

use crate::models::errors::ErrorResponse;

impl From<DatabaseCreationError> for ErrorResponse {
    fn from(error: DatabaseCreationError) -> Self {
        match error {
            DatabaseCreationError::AlreadyExists => ErrorResponse {
                status_code: StatusCode::CONFLICT.as_u16(),
                message: String::from("Error creating Database: Database already exists"),
            },
            DatabaseCreationError::InvalidId => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                message: String::from("Error creating Database: Invalid Database ID"),
            },
            DatabaseCreationError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: format!("Error creating Database: {e}"),
            },
        }
    }
}
