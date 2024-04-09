use actix_web::http::StatusCode;
use furdb_core::errors::entry_errors::entry_deletion_error::EntryDeletionError;

use crate::models::response::error_response::ErrorResponse;

impl From<EntryDeletionError> for ErrorResponse {
    fn from(error: EntryDeletionError) -> Self {
        match error {
            EntryDeletionError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: format!("Deletion Query Error: {e}"),
            },
        }
    }
}
