use std::{error::Error, sync::Arc};

use nimbus_vault_server_domain::entities::user::{User, specifications::NewUserSpecification};

use crate::{
    services::{
        crypto::CryptoService, repositories::user_repository::UserRepository, time::TimeService,
    },
    use_cases::user::UserSchema,
};

pub struct RegisterUserRequestSchema {
    pub username: String,
    pub password: String,
    pub e2e_key_hash: String,
    pub encrypted_master_key: String,
}

pub struct RegisterUserResponseSchema {
    pub user: UserSchema,
}

pub struct RegisterUserUseCase {
    user_repository: Arc<dyn UserRepository>,
    time_service: Arc<dyn TimeService>,
    crypto_service: Arc<dyn CryptoService>,
}

impl RegisterUserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        time_service: Arc<dyn TimeService>,
        crypto_service: Arc<dyn CryptoService>,
    ) -> Self {
        Self {
            user_repository,
            time_service,
            crypto_service,
        }
    }

    pub async fn execute(
        &self,
        RegisterUserRequestSchema {
            username,
            password,
            e2e_key_hash,
            encrypted_master_key,
        }: RegisterUserRequestSchema,
    ) -> Result<RegisterUserResponseSchema, Box<dyn Error>> {
        let password_hash = self.crypto_service.get_password_hash(password).await;
        let current_time = self.time_service.get_current_time().await?;
        let user = User::new(NewUserSpecification {
            username,
            password_hash,
            e2e_key_hash,
            encrypted_master_key,
            current_time,
        })?;
        self.user_repository.save(&user).await?;
        Ok(RegisterUserResponseSchema {
            user: UserSchema {
                id: user.id().to_string(),
                username: user.username().to_owned(),
                encrypted_master_key: user.encrypted_master_key().to_owned(),
            },
        })
    }
}
