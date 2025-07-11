use async_trait::async_trait;
use nimbus_vault_server_application::services::repositories::UserRepository;
use nimbus_vault_server_domain::entities::User;

use std::{error::Error, sync::Arc};

use crate::database::{Database, db_entities::user::UserDb};

pub struct DefaultUserRepository {
    database: Arc<Database>,
}

impl DefaultUserRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl UserRepository for DefaultUserRepository {
    async fn get_by_id(&self, id: ulid::Ulid) -> Result<Option<User>, Box<dyn Error>> {
        let id_str = id.to_string();
        let user_db = sqlx::query_as!(
            UserDb,
            r#"
            select id, username, password_hash, e2e_key_hash, encrypted_master_key, created_at, updated_at
            from users
            where id = $1
            "#,
            id_str
        )
        .fetch_optional(self.database.pool())
        .await?;

        todo!()
    }

    async fn save(&self, user: User) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
