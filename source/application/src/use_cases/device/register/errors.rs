use crate::{
    services::{
        auth::errors::AuthServiceError,
        crypto::errors::CryptoServiceError,
        errors::ServiceError,
        repositories::{
            device_repository::errors::DeviceRepositoryError, errors::RepositoryError,
            user_repository::errors::UserRepositoryError,
        },
        time::errors::TimeServiceError,
    },
    use_cases::user::register::errors::RegisterUserUseCaseError,
};
use nimbus_vault_server_domain::entities::{
    device::errors::DeviceError,
    user::{errors::UserError, value_objects::errors::UserPasswordError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterDeviceUseCaseError {
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    User(#[from] UserError),
    #[error("User not found")]
    UserNotFound,
    #[error("Password is wrong")]
    WrongPassword,
    #[error("E2E key is wrong")]
    WrongE2EKey,
}

impl From<UserPasswordError> for RegisterDeviceUseCaseError {
    fn from(value: UserPasswordError) -> Self {
        RegisterDeviceUseCaseError::from(UserError::from(value))
    }
}

impl From<UserRepositoryError> for RegisterDeviceUseCaseError {
    fn from(value: UserRepositoryError) -> Self {
        RegisterDeviceUseCaseError::from(ServiceError::from(RepositoryError::from(value)))
    }
}

impl From<DeviceRepositoryError> for RegisterDeviceUseCaseError {
    fn from(value: DeviceRepositoryError) -> Self {
        RegisterDeviceUseCaseError::from(ServiceError::from(RepositoryError::from(value)))
    }
}

impl From<CryptoServiceError> for RegisterDeviceUseCaseError {
    fn from(value: CryptoServiceError) -> Self {
        RegisterDeviceUseCaseError::from(ServiceError::from(value))
    }
}

impl From<AuthServiceError> for RegisterDeviceUseCaseError {
    fn from(value: AuthServiceError) -> Self {
        RegisterDeviceUseCaseError::from(ServiceError::from(value))
    }
}

impl From<TimeServiceError> for RegisterDeviceUseCaseError {
    fn from(value: TimeServiceError) -> Self {
        RegisterDeviceUseCaseError::from(ServiceError::from(value))
    }
}
