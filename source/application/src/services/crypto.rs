pub mod errors;

use async_trait::async_trait;

use crate::services::crypto::errors::CryptoServiceError;

#[async_trait]
pub trait CryptoService: Send + Sync {
    async fn get_password_hash(&self, password: String) -> Result<String, CryptoServiceError>;
}
