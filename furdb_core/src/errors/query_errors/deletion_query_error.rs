use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeletionQueryError {
    #[error("Deletion Query Error")]
    OtherError(String),
}