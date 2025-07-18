use async_trait::async_trait;
use nimbus_vault_server_application::services::repositories::user_repository::UserRepository;
use nimbus_vault_server_domain::entities::user::{User, specifications::RestoreUserSpecification};
use ulid::Ulid;

use std::{error::Error, sync::Arc};

use crate::database::{Database, db_entities::user::UserDb};

pub struct DefaultUserRepository {
    database: Arc<Database>,
}

impl DefaultUserRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    fn to_db_entity(&self, user: &User) -> UserDb {
        UserDb {
            id: user.id().to_string(),
            username: user.username().to_string(),
            password_hash: user.password_hash().to_string(),
            e2e_key_hash: user.e2e_key_hash().to_string(),
            encrypted_master_key: user.encrypted_master_key().to_string(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }

    fn to_domain_entity(&self, user_db: UserDb) -> Result<User, Box<dyn Error>> {
        User::restore(RestoreUserSpecification {
            id: Ulid::from_string(user_db.id.as_str())?,
            username: user_db.username,
            password_hash: user_db.password_hash,
            e2e_key_hash: user_db.e2e_key_hash,
            encrypted_master_key: user_db.encrypted_master_key,
            created_at: user_db.created_at,
            updated_at: user_db.updated_at,
        })
    }
}

#[async_trait]
impl UserRepository for DefaultUserRepository {
    async fn get_by_id(&self, id: ulid::Ulid) -> Result<Option<User>, Box<dyn Error>> {
        let id_str = id.to_string();
        sqlx::query_as!(
            UserDb,
            r#"
            select id, username, password_hash, e2e_key_hash, encrypted_master_key, created_at, updated_at
            from users
            where id = $1
            "#,
            id_str
        )
        .fetch_optional(self.database.pool())
        .await?
        .map(|user_db| self.to_domain_entity(user_db))
        .transpose()
    }

    async fn save(&self, user: &User) -> Result<(), Box<dyn Error>> {
        let user_db = self.to_db_entity(user);
        sqlx::query!(
            r#"
            insert into users
            (id, username, password_hash, e2e_key_hash, encrypted_master_key, created_at, updated_at)
            values 
            ($1, $2, $3, $4, $5, $6, $7)
            on conflict (id) do update
            set username = excluded.username,
                password_hash = excluded.password_hash,
                e2e_key_hash = excluded.e2e_key_hash,
                encrypted_master_key = excluded.encrypted_master_key,
                updated_at = excluded.updated_at
            "#,
            user_db.id,
            user_db.username,
            user_db.password_hash,
            user_db.e2e_key_hash,
            user_db.encrypted_master_key,
            user_db.created_at,
            user_db.updated_at
        )
        .execute(self.database.pool())
        .await?;
        Ok(())
    }
}
