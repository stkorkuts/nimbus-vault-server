use std::{error::Error, sync::Arc};

use crate::{
    services::{crypto::CryptoService, repositories::UserRepository, time::TimeService},
    use_cases::{ApplicationUseCases, user::register::RegisterUserUseCase},
};

pub struct ApplicationUseCasesBuilder {
    user_repository: Option<Arc<dyn UserRepository>>,
    time_service: Option<Arc<dyn TimeService>>,
    crypto_service: Option<Arc<dyn CryptoService>>,
}

impl ApplicationUseCasesBuilder {
    pub fn new() -> Self {
        ApplicationUseCasesBuilder {
            user_repository: None,
            time_service: None,
            crypto_service: None,
        }
    }

    pub fn with_user_repository(&mut self, user_repository: Arc<dyn UserRepository>) -> &mut Self {
        self.user_repository = Some(user_repository);
        self
    }

    pub fn with_time_service(&mut self, time_service: Arc<dyn TimeService>) -> &mut Self {
        self.time_service = Some(time_service);
        self
    }

    pub fn with_crypto_service(&mut self, crypto_service: Arc<dyn CryptoService>) -> &mut Self {
        self.crypto_service = Some(crypto_service);
        self
    }

    pub fn build(&self) -> Result<ApplicationUseCases, Box<dyn Error>> {
        let user_repository = self
            .user_repository
            .as_ref()
            .ok_or("User repository is required")?;
        let time_service = self
            .time_service
            .as_ref()
            .ok_or("Time service is required")?;
        let crypto_service = self
            .crypto_service
            .as_ref()
            .ok_or("Crypto service is required")?;

        Ok(ApplicationUseCases {
            register_user: RegisterUserUseCase::new(
                user_repository.clone(),
                time_service.clone(),
                crypto_service.clone(),
            ),
        })
    }
}
