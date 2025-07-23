use nimbus_vault_server_domain::value_objects::password_hash::errors::PasswordHashAlgorithmError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoServiceError {
    #[error(transparent)]
    PasswordHash(#[from] PasswordHashAlgorithmError),
    #[error("Failed to hash password: {error_message}")]
    HashingFailed { error_message: String },
    #[error("Invalid password hash format: {error_message}")]
    InvalidFormat { error_message: String },
}
