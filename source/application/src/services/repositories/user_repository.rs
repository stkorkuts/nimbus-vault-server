use std::error::Error;

use async_trait::async_trait;
use nimbus_vault_server_domain::entities::user::User;
use ulid::Ulid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_id(&self, id: Ulid) -> Result<Option<User>, Box<dyn Error>>;
    async fn save(&self, user: &User) -> Result<(), Box<dyn Error>>;
}
