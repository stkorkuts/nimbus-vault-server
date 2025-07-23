use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordHashAlgorithmError {
    #[error("Invalid password hash format: {error_message}")]
    InvalidFormat { error_message: String },
    #[error("Unsupported password hash algorithm: {algorithm}")]
    UnsupportedAlgorithm { algorithm: String },
}
