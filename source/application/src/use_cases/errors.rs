use crate::use_cases::{
    builder::errors::UseCasesBuilderError, device::errors::DeviceUseCaseError,
    user::errors::UserUseCaseError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error(transparent)]
    Builder(#[from] UseCasesBuilderError),
    #[error(transparent)]
    User(#[from] UserUseCaseError),
    #[error(transparent)]
    Device(#[from] DeviceUseCaseError),
}
