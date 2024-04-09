use actix_web::http::StatusCode;
use furdb_core::errors::database_errors::database_deletion_error::DatabaseDeletionError;

use crate::models::response::error_response::ErrorResponse;

impl From<DatabaseDeletionError> for ErrorResponse {
    fn from(error: DatabaseDeletionError) -> Self {
        match error {
            DatabaseDeletionError::NotFound => ErrorResponse {
                status_code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            },
            DatabaseDeletionError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: format!("Error deleting Database: {e}"),
            },
        }
    }
}
