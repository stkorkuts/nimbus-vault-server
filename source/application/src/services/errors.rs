use crate::services::{
    auth::errors::AuthServiceError, crypto::errors::CryptoServiceError,
    repositories::errors::RepositoryError, time::errors::TimeServiceError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error(transparent)]
    AuthService(#[from] AuthServiceError),
    #[error(transparent)]
    CryptoService(#[from] CryptoServiceError),
    #[error(transparent)]
    TimeService(#[from] TimeServiceError),
}
