use actix_web::http::StatusCode;
use furdb_core::errors::table_errors::table_deletion_error::TableDeletionError;

use crate::models::response::error_response::ErrorResponse;

impl From<TableDeletionError> for ErrorResponse {
    fn from(error: TableDeletionError) -> Self {
        match error {
            TableDeletionError::NotFound => ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: error.to_string(),
            },
            TableDeletionError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: format!("Error deleting Table: {e}"),
            },
        }
    }
}
