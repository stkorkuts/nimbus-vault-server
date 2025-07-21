use crate::services::{
    errors::ServiceError,
    repositories::{device_repository::errors::DeviceRepositoryError, errors::RepositoryError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterDeviceUseCaseError {
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[error("User not found")]
    UserNotFound,
    #[error("Password is wrong")]
    PasswordIsWrong,
}

impl From<DeviceRepositoryError> for RegisterDeviceUseCaseError {
    fn from(value: DeviceRepositoryError) -> Self {
        RegisterDeviceUseCaseError::Service(ServiceError::from(RepositoryError::from(value)))
    }
}
