use std::{error::Error, fmt::Display};

use crate::entities::{device::errors::DeviceError, user::errors::UserError};

#[derive(Debug)]
pub enum DomainError {
    User(UserError),
    Device(DeviceError),
}

impl Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User(user_error) => write!(f, "Domain user error: {}", user_error),
            Self::Device(device_error) => write!(f, "Domain device error: {}", device_error),
        }
    }
}

impl Error for DomainError {}

impl From<UserError> for DomainError {
    fn from(value: UserError) -> Self {
        DomainError::User(value)
    }
}

impl From<DeviceError> for DomainError {
    fn from(value: DeviceError) -> Self {
        DomainError::Device(value)
    }
}
