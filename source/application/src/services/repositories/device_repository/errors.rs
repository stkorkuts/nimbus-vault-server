use nimbus_vault_server_domain::entities::device::errors::DeviceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceRepositoryError {
    #[error("Data source error: {error_message}")]
    DataSource { error_message: String },
    #[error("Error decoding values: {error_message}")]
    Decode { error_message: String },
    #[error(transparent)]
    Device(#[from] DeviceError),
}
