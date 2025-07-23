use nimbus_vault_server_domain::entities::user::errors::UserError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Data source error: {error_message}")]
    DataSource { error_message: String },
    #[error("Error decoding values: {error_message}")]
    Decode { error_message: String },
    #[error(transparent)]
    User(#[from] UserError),
}
