pub mod errors;

use crate::services::crypto::errors::CryptoServiceError;
use async_trait::async_trait;
use nimbus_vault_server_domain::{
    entities::user::value_objects::UserPassword,
    value_objects::password_hash::PasswordHashAlgorithm,
};

#[async_trait]
pub trait CryptoService: Send + Sync {
    async fn get_user_password_hash(
        &self,
        password: &UserPassword,
        algorithm: Option<PasswordHashAlgorithm>,
    ) -> Result<String, CryptoServiceError>;
    async fn verify_user_password(
        &self,
        password: &UserPassword,
        stored_hash: &str,
    ) -> Result<bool, CryptoServiceError>;
}
