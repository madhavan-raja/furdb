use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseCreationError {
    #[error("Error creating Database: Database already exists")]
    AlreadyExists,
    #[error("Error creating Database: Invalid ID")]
    InvalidId,
    #[error("Error creating Database")]
    OtherError(String),
}