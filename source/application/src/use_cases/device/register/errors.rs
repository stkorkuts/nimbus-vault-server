use crate::services::{
    auth::errors::AuthServiceError,
    crypto::errors::CryptoServiceError,
    errors::ServiceError,
    repositories::{
        device_repository::errors::DeviceRepositoryError, errors::RepositoryError,
        user_repository::errors::UserRepositoryError,
    },
    time::errors::TimeServiceError,
};
use nimbus_vault_server_domain::entities::device::errors::DeviceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterDeviceUseCaseError {
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("User not found")]
    UserNotFound,
    #[error("Password is wrong")]
    WrongPassword,
    #[error("E2E key is wrong")]
    WrongE2EKey,
}

impl From<UserRepositoryError> for RegisterDeviceUseCaseError {
    fn from(value: UserRepositoryError) -> Self {
        RegisterDeviceUseCaseError::Service(ServiceError::from(RepositoryError::from(value)))
    }
}

impl From<DeviceRepositoryError> for RegisterDeviceUseCaseError {
    fn from(value: DeviceRepositoryError) -> Self {
        RegisterDeviceUseCaseError::Service(ServiceError::from(RepositoryError::from(value)))
    }
}

impl From<CryptoServiceError> for RegisterDeviceUseCaseError {
    fn from(value: CryptoServiceError) -> Self {
        RegisterDeviceUseCaseError::Service(ServiceError::from(value))
    }
}

impl From<AuthServiceError> for RegisterDeviceUseCaseError {
    fn from(value: AuthServiceError) -> Self {
        RegisterDeviceUseCaseError::Service(ServiceError::from(value))
    }
}

impl From<TimeServiceError> for RegisterDeviceUseCaseError {
    fn from(value: TimeServiceError) -> Self {
        RegisterDeviceUseCaseError::Service(ServiceError::from(value))
    }
}
