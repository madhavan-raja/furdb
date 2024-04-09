use actix_web::http::StatusCode;
use furdb_core::errors::table_errors::table_creation_error::TableCreationError;

use crate::models::response::error_response::ErrorResponse;

impl From<TableCreationError> for ErrorResponse {
    fn from(error: TableCreationError) -> Self {
        match error {
            TableCreationError::AlreadyExists => ErrorResponse {
                status_code: StatusCode::CONFLICT.as_u16(),
                error: error.to_string(),
            },
            TableCreationError::InvalidId => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error: error.to_string(),
            },
            TableCreationError::ColumnsUnfit => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error: error.to_string(),
            },
            TableCreationError::OtherError(e) => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: format!("Error creating Table: {e}"),
            },
        }
    }
}