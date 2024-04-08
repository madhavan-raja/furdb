use thiserror::Error;

#[derive(Error, Debug)]
pub enum FurDBInitializationError {
    #[error("Error initializing FurDB: Permission denied")]
    PermissionDenied,
    #[error("Error initializing FurDB: Invalid path")]
    InvalidPath,
    #[error("Error initializing FurDB")]
    OtherError(String),
}