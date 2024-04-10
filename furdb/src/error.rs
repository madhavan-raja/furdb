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

impl From<DatabaseCreationError> for ErrorResponse {
    fn from(error: DatabaseCreationError) -> Self {
        match error {
            DatabaseCreationError::AlreadyExists => ErrorResponse::Conflict(error.to_string()),
            DatabaseCreationError::InvalidId => ErrorResponse::BadRequest(error.to_string()),
            DatabaseCreationError::OtherError(_e) => ErrorResponse::InternalServerError,
        }
    }
}

impl From<DatabaseDeletionError> for ErrorResponse {
    fn from(error: DatabaseDeletionError) -> Self {
        match error {
            DatabaseDeletionError::NotFound => ErrorResponse::NotFound(error.to_string()),
            DatabaseDeletionError::OtherError(_e) => ErrorResponse::InternalServerError,
        }
    }
}

impl From<DatabaseReadError> for ErrorResponse {
    fn from(error: DatabaseReadError) -> Self {
        match error {
            DatabaseReadError::NotFound => ErrorResponse::NotFound(error.to_string()),
            DatabaseReadError::OtherError(_e) => ErrorResponse::InternalServerError,
        }
    }
}

impl From<TableCreationError> for ErrorResponse {
    fn from(error: TableCreationError) -> Self {
        match error {
            TableCreationError::AlreadyExists => ErrorResponse::Conflict(error.to_string()),
            TableCreationError::InvalidId => ErrorResponse::BadRequest(error.to_string()),
            TableCreationError::ColumnsUnfit => ErrorResponse::BadRequest(error.to_string()),
            TableCreationError::OtherError(_e) => ErrorResponse::InternalServerError,
        }
    }
}

impl From<TableDeletionError> for ErrorResponse {
    fn from(error: TableDeletionError) -> Self {
        match error {
            TableDeletionError::NotFound => ErrorResponse::NotFound(error.to_string()),
            TableDeletionError::OtherError(_e) => ErrorResponse::InternalServerError,
        }
    }
}

impl From<TableReadError> for ErrorResponse {
    fn from(error: TableReadError) -> Self {
        match error {
            TableReadError::NotFound => ErrorResponse::NotFound(error.to_string()),
            TableReadError::OtherError(_e) => ErrorResponse::InternalServerError,
        }
    }
}

impl From<EntryInsertionError> for ErrorResponse {
    fn from(error: EntryInsertionError) -> Self {
        match error {
            EntryInsertionError::ColumnMismatch => ErrorResponse::BadRequest(error.to_string()),
            EntryInsertionError::ColumnOverflow => ErrorResponse::BadRequest(error.to_string()),
            EntryInsertionError::OtherError(_e) => ErrorResponse::InternalServerError,
        }
    }
}

impl From<EntryDeletionError> for ErrorResponse {
    fn from(error: EntryDeletionError) -> Self {
        match error {
            EntryDeletionError::OtherError(_e) => ErrorResponse::InternalServerError,
        }
    }
}

impl From<EntryReadError> for ErrorResponse {
    fn from(error: EntryReadError) -> Self {
        match error {
            EntryReadError::InvalidIndex => ErrorResponse::BadRequest(error.to_string()),
            EntryReadError::InvalidColumn => ErrorResponse::BadRequest(error.to_string()),
            EntryReadError::OtherError(_e) => ErrorResponse::InternalServerError,
        }
    }
}
