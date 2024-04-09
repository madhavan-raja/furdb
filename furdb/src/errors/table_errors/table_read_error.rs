use actix_web::http::StatusCode;
use furdb_core::errors::table_errors::table_read_error::TableReadError;

use crate::models::response::error_response::ErrorResponse;

impl From<TableReadError> for ErrorResponse {
    fn from(error: TableReadError) -> Self {
        match error {
            TableReadError::NotFound => ErrorResponse {
                status_code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            },
            TableReadError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: format!("Error reading Table: {e}"),
            },
        }
    }
}
