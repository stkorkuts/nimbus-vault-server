pub mod errors;

use async_trait::async_trait;
use nimbus_vault_server_domain::entities::user::User;
use ulid::Ulid;

use crate::services::repositories::user_repository::errors::UserRepositoryError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_id(&self, id: Ulid) -> Result<Option<User>, UserRepositoryError>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, UserRepositoryError>;
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
}
