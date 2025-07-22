use thiserror::Error;

use crate::entities::{device::errors::DeviceError, user::errors::UserError};

#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    User(#[from] UserError),
    #[error(transparent)]
    Device(#[from] DeviceError),
}
