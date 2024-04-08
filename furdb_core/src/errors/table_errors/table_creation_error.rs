use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableCreationError {
    #[error("Error creating Table: Table already exists")]
    AlreadyExists,
    #[error("Error creating Table: Invalid Table ID")]
    InvalidId,
    #[error("Error creating Table: Size of row must be a multiple of 8")]
    ColumnsUnfit,
    #[error("Error creating Table")]
    OtherError(String),
}