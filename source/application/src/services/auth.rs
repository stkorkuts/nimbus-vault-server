pub mod errors;

use async_trait::async_trait;
use nimbus_vault_server_domain::entities::{device::Device, user::User};

use crate::services::auth::errors::AuthServiceError;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn sign_certificate(&self, request: Vec<u8>) -> Result<Vec<u8>, AuthServiceError>;
    async fn get_certificate_fingerprint(
        &self,
        certificate: Vec<u8>,
    ) -> Result<String, AuthServiceError>;
    async fn get_auth_by_fingerprint(
        &self,
        certificate_fingerprint: String,
    ) -> Result<(User, Device), AuthServiceError>;
}
