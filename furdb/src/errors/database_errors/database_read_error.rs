use actix_web::http::StatusCode;
use furdb_core::errors::database_errors::database_read_error::DatabaseReadError;

use crate::models::response::error_response::ErrorResponse;

impl From<DatabaseReadError> for ErrorResponse {
    fn from(error: DatabaseReadError) -> Self {
        match error {
            DatabaseReadError::NotFound => ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: error.to_string(),
            },
            DatabaseReadError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: format!("Error reading Database: {e}"),
            },
        }
    }
}
