use crate::use_cases::device::register::errors::RegisterDeviceUseCaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceUseCaseError {
    #[error(transparent)]
    Register(#[from] RegisterDeviceUseCaseError),
}
