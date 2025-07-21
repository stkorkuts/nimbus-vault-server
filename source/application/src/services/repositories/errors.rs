use crate::services::repositories::{
    device_repository::errors::DeviceRepositoryError, user_repository::errors::UserRepositoryError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error(transparent)]
    User(#[from] UserRepositoryError),
    #[error(transparent)]
    Device(#[from] DeviceRepositoryError),
}
