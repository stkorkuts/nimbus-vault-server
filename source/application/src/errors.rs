use crate::{services::errors::ServiceError, use_cases::errors::UseCaseError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[error(transparent)]
    UseCase(#[from] UseCaseError),
}
