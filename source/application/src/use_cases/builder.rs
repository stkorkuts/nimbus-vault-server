pub mod errors;

use std::sync::Arc;

use crate::{
    services::{
        auth::AuthService,
        crypto::CryptoService,
        repositories::{
            device_repository::{self, DeviceRepository},
            user_repository::UserRepository,
        },
        time::TimeService,
    },
    use_cases::{
        ApplicationUseCases, builder::errors::UseCasesBuilderError,
        device::register::RegisterDeviceUseCase, user::register::RegisterUserUseCase,
    },
};

pub struct ApplicationUseCasesBuilder {
    user_repository: Option<Arc<dyn UserRepository>>,
    device_repository: Option<Arc<dyn DeviceRepository>>,
    time_service: Option<Arc<dyn TimeService>>,
    crypto_service: Option<Arc<dyn CryptoService>>,
    auth_service: Option<Arc<dyn AuthService>>,
}

impl ApplicationUseCasesBuilder {
    pub fn new() -> Self {
        ApplicationUseCasesBuilder {
            user_repository: None,
            device_repository: None,
            time_service: None,
            crypto_service: None,
            auth_service: None,
        }
    }

    pub fn with_user_repository(&mut self, user_repository: Arc<dyn UserRepository>) -> &mut Self {
        self.user_repository = Some(user_repository);
        self
    }

    pub fn with_device_repository(
        &mut self,
        device_repository: Arc<dyn DeviceRepository>,
    ) -> &mut Self {
        self.device_repository = Some(device_repository);
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

    pub fn with_auth_service(&mut self, auth_service: Arc<dyn AuthService>) -> &mut Self {
        self.auth_service = Some(auth_service);
        self
    }

    pub fn build(&self) -> Result<ApplicationUseCases, UseCasesBuilderError> {
        let user_repository =
            self.user_repository
                .as_ref()
                .ok_or(UseCasesBuilderError::ServiceIsMissing {
                    service_name: "UserRepository".to_owned(),
                })?;
        let device_repository =
            self.device_repository
                .as_ref()
                .ok_or(UseCasesBuilderError::ServiceIsMissing {
                    service_name: "DeviceRepository".to_owned(),
                })?;
        let time_service =
            self.time_service
                .as_ref()
                .ok_or(UseCasesBuilderError::ServiceIsMissing {
                    service_name: "TimeService".to_owned(),
                })?;
        let crypto_service =
            self.crypto_service
                .as_ref()
                .ok_or(UseCasesBuilderError::ServiceIsMissing {
                    service_name: "CryptoService".to_owned(),
                })?;
        let auth_service =
            self.auth_service
                .as_ref()
                .ok_or(UseCasesBuilderError::ServiceIsMissing {
                    service_name: "AuthService".to_owned(),
                })?;

        Ok(ApplicationUseCases {
            register_user: RegisterUserUseCase::new(
                user_repository.clone(),
                time_service.clone(),
                crypto_service.clone(),
            ),
            register_device: RegisterDeviceUseCase::new(
                user_repository.clone(),
                device_repository.clone(),
                time_service.clone(),
                crypto_service.clone(),
                auth_service.clone(),
            ),
        })
    }
}
