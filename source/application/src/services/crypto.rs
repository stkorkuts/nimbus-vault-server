pub mod errors;

use async_trait::async_trait;
use nimbus_vault_server_domain::entities::user::value_objects::UserPassword;

use crate::services::crypto::errors::CryptoServiceError;

#[async_trait]
pub trait CryptoService: Send + Sync {
    async fn get_user_password_hash(
        &self,
        password: &UserPassword,
    ) -> Result<String, CryptoServiceError>;
}
