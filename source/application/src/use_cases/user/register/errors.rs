use nimbus_vault_server_domain::entities::user::{
    errors::UserError, value_objects::errors::UserPasswordError,
};
use thiserror::Error;

use crate::services::{
    crypto::errors::CryptoServiceError,
    errors::ServiceError,
    repositories::{errors::RepositoryError, user_repository::errors::UserRepositoryError},
    time::errors::TimeServiceError,
};

#[derive(Debug, Error)]
pub enum RegisterUserUseCaseError {
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[error(transparent)]
    User(#[from] UserError),
    #[error("User with this username already exists")]
    UserAlreadyExists,
}

impl From<UserRepositoryError> for RegisterUserUseCaseError {
    fn from(value: UserRepositoryError) -> Self {
        RegisterUserUseCaseError::from(ServiceError::from(RepositoryError::from(value)))
    }
}

impl From<UserPasswordError> for RegisterUserUseCaseError {
    fn from(value: UserPasswordError) -> Self {
        RegisterUserUseCaseError::from(UserError::from(value))
    }
}

impl From<CryptoServiceError> for RegisterUserUseCaseError {
    fn from(value: CryptoServiceError) -> Self {
        RegisterUserUseCaseError::from(ServiceError::from(value))
    }
}

impl From<TimeServiceError> for RegisterUserUseCaseError {
    fn from(value: TimeServiceError) -> Self {
        RegisterUserUseCaseError::from(ServiceError::from(value))
    }
}
