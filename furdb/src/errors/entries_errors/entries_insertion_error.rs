use actix_web::http::StatusCode;
use furdb_core::errors::entry_errors::entry_insertion_error::EntryInsertionError;

use crate::models::response::error_response::ErrorResponse;

impl From<EntryInsertionError> for ErrorResponse {
    fn from(error: EntryInsertionError) -> Self {
        match error {
            EntryInsertionError::ColumnMismatch => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                error: error.to_string(),
            },
            EntryInsertionError::ColumnOverflow => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                error: error.to_string(),
            },
            EntryInsertionError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: format!("Insertion Query Error: {e}"),
            },
        }
    }
}
