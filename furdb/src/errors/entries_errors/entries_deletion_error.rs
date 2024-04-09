use actix_web::http::StatusCode;
use furdb_core::errors::query_errors::deletion_query_error::DeletionQueryError;

use crate::models::response::error_response::ErrorResponse;

impl From<DeletionQueryError> for ErrorResponse {
    fn from(error: DeletionQueryError) -> Self {
        match error {
            DeletionQueryError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: format!("Deletion Query Error: {e}"),
            },
        }
    }
}
