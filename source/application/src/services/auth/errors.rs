use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Invalid certificate signing request (CSR)")]
    InvalidCsr,
    #[error("Invalid certificate data")]
    InvalidCertificate,
    #[error("Certificate signing failed: {0}")]
    SigningFailed(String),
}
