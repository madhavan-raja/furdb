use crate::models::response::api_response::ApiResponse;
use crate::models::response::error_response::ErrorResponse;

use furdb_core::errors;

use errors::database_errors;
use errors::entry_errors;
use errors::table_errors;

use database_errors::database_creation_error::DatabaseCreationError;
use database_errors::database_deletion_error::DatabaseDeletionError;
use database_errors::database_read_error::DatabaseReadError;

use table_errors::table_creation_error::TableCreationError;
use table_errors::table_deletion_error::TableDeletionError;
use table_errors::table_read_error::TableReadError;

use entry_errors::entry_deletion_error::EntryDeletionError;
use entry_errors::entry_insertion_error::EntryInsertionError;
use entry_errors::entry_read_error::EntryReadError;

impl From<DatabaseCreationError> for ApiResponse {
    fn from(error: DatabaseCreationError) -> Self {
        match error {
            DatabaseCreationError::AlreadyExists => {
                ApiResponse::Error(ErrorResponse::Conflict(error.to_string()))
            }
            DatabaseCreationError::InvalidId => {
                ApiResponse::Error(ErrorResponse::BadRequest(error.to_string()))
            }
            DatabaseCreationError::OtherError(_e) => {
                ApiResponse::Error(ErrorResponse::InternalServerError)
            }
        }
    }
}

impl From<DatabaseDeletionError> for ApiResponse {
    fn from(error: DatabaseDeletionError) -> Self {
        match error {
            DatabaseDeletionError::NotFound => {
                ApiResponse::Error(ErrorResponse::NotFound(error.to_string()))
            }
            DatabaseDeletionError::OtherError(_e) => {
                ApiResponse::Error(ErrorResponse::InternalServerError)
            }
        }
    }
}

impl From<DatabaseReadError> for ApiResponse {
    fn from(error: DatabaseReadError) -> Self {
        match error {
            DatabaseReadError::NotFound => {
                ApiResponse::Error(ErrorResponse::NotFound(error.to_string()))
            }
            DatabaseReadError::OtherError(_e) => {
                ApiResponse::Error(ErrorResponse::InternalServerError)
            }
        }
    }
}

impl From<TableCreationError> for ApiResponse {
    fn from(error: TableCreationError) -> Self {
        match error {
            TableCreationError::AlreadyExists => {
                ApiResponse::Error(ErrorResponse::Conflict(error.to_string()))
            }
            TableCreationError::InvalidId => {
                ApiResponse::Error(ErrorResponse::BadRequest(error.to_string()))
            }
            TableCreationError::ColumnsUnfit => {
                ApiResponse::Error(ErrorResponse::BadRequest(error.to_string()))
            }
            TableCreationError::OtherError(_e) => {
                ApiResponse::Error(ErrorResponse::InternalServerError)
            }
        }
    }
}

impl From<TableDeletionError> for ApiResponse {
    fn from(error: TableDeletionError) -> Self {
        match error {
            TableDeletionError::NotFound => {
                ApiResponse::Error(ErrorResponse::NotFound(error.to_string()))
            }
            TableDeletionError::OtherError(_e) => {
                ApiResponse::Error(ErrorResponse::InternalServerError)
            }
        }
    }
}

impl From<TableReadError> for ApiResponse {
    fn from(error: TableReadError) -> Self {
        match error {
            TableReadError::NotFound => {
                ApiResponse::Error(ErrorResponse::NotFound(error.to_string()))
            }
            TableReadError::OtherError(_e) => {
                ApiResponse::Error(ErrorResponse::InternalServerError)
            }
        }
    }
}

impl From<EntryInsertionError> for ApiResponse {
    fn from(error: EntryInsertionError) -> Self {
        match error {
            EntryInsertionError::ColumnMismatch => {
                ApiResponse::Error(ErrorResponse::BadRequest(error.to_string()))
            }
            EntryInsertionError::ColumnOverflow => {
                ApiResponse::Error(ErrorResponse::BadRequest(error.to_string()))
            }
            EntryInsertionError::OtherError(_e) => {
                ApiResponse::Error(ErrorResponse::InternalServerError)
            }
        }
    }
}

impl From<EntryDeletionError> for ApiResponse {
    fn from(error: EntryDeletionError) -> Self {
        match error {
            EntryDeletionError::OtherError(_e) => {
                ApiResponse::Error(ErrorResponse::InternalServerError)
            }
        }
    }
}

impl From<EntryReadError> for ApiResponse {
    fn from(error: EntryReadError) -> Self {
        match error {
            EntryReadError::InvalidIndex => {
                ApiResponse::Error(ErrorResponse::BadRequest(error.to_string()))
            }
            EntryReadError::InvalidColumn => {
                ApiResponse::Error(ErrorResponse::BadRequest(error.to_string()))
            }
            EntryReadError::OtherError(_e) => {
                ApiResponse::Error(ErrorResponse::InternalServerError)
            }
        }
    }
}
