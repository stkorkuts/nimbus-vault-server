use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Invalid device name. Error: {error_message}")]
    InvalidName { error_message: String },
}
