use actix_web::http::StatusCode;
use furdb_core::errors::query_errors::insertion_query_error::InsertionQueryError;

use crate::models::response::error_response::ErrorResponse;

impl From<InsertionQueryError> for ErrorResponse {
    fn from(error: InsertionQueryError) -> Self {
        match error {
            InsertionQueryError::ColumnMismatch => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error: error.to_string(),
            },
            InsertionQueryError::ColumnOverflow => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error: error.to_string(),
            },
            InsertionQueryError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: format!("Insertion Query Error: {e}"),
            },
        }
    }
}
