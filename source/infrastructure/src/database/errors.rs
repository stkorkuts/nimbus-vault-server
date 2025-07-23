use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),
}
