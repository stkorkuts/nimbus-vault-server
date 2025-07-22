pub mod errors;

use std::{error::Error, sync::Arc};

use nimbus_vault_server_domain::{
    entities::device::{Device, specifications::NewDeviceSpecification},
    enums::DeviceType,
};

use crate::{
    errors::ApplicationError,
    services::{
        auth::AuthService,
        crypto::CryptoService,
        errors::ServiceError,
        repositories::{
            device_repository::DeviceRepository, errors::RepositoryError,
            user_repository::UserRepository,
        },
        time::TimeService,
    },
    use_cases::{
        device::{DeviceSchema, register::errors::RegisterDeviceUseCaseError},
        user::UserSchema,
    },
};

pub struct RegisterDeviceRequestSchema {
    pub username: String,
    pub password: String,
    pub e2e_key_hash: String,
    pub certificate_sign_request: Vec<u8>,
    pub device_name: String,
    pub device_type: DeviceType,
}

pub struct RegisterDeviceResponeSchema {
    pub user: UserSchema,
    pub device: DeviceSchema,
    pub signed_certificate: Vec<u8>,
}

pub struct RegisterDeviceUseCase {
    user_repository: Arc<dyn UserRepository>,
    device_repository: Arc<dyn DeviceRepository>,
    time_service: Arc<dyn TimeService>,
    crypto_service: Arc<dyn CryptoService>,
    auth_service: Arc<dyn AuthService>,
}

impl RegisterDeviceUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        device_repository: Arc<dyn DeviceRepository>,
        time_service: Arc<dyn TimeService>,
        crypto_service: Arc<dyn CryptoService>,
        auth_service: Arc<dyn AuthService>,
    ) -> Self {
        RegisterDeviceUseCase {
            user_repository,
            device_repository,
            time_service,
            crypto_service,
            auth_service,
        }
    }

    pub async fn execute(
        &self,
        RegisterDeviceRequestSchema {
            username,
            password,
            e2e_key_hash,
            certificate_sign_request,
            device_name,
            device_type,
        }: RegisterDeviceRequestSchema,
    ) -> Result<RegisterDeviceResponeSchema, RegisterDeviceUseCaseError> {
        let user = self
            .user_repository
            .get_by_username(&username)
            .await?
            .ok_or(RegisterDeviceUseCaseError::UserNotFound)?;
        let password_hash = self.crypto_service.get_password_hash(&password).await?;
        if password_hash != user.password_hash() {
            return Err(RegisterDeviceUseCaseError::WrongPassword);
        }
        if e2e_key_hash != user.e2e_key_hash() {
            return Err(RegisterDeviceUseCaseError::WrongE2EKey);
        }
        let signed_certificate = self
            .auth_service
            .sign_certificate(&certificate_sign_request)
            .await?;
        let certificate_fingerprint = self
            .auth_service
            .get_certificate_fingerprint(&signed_certificate)
            .await?;
        let current_time = self.time_service.get_current_time().await?;
        let device = Device::new(NewDeviceSpecification {
            user_id: user.id(),
            name: device_name,
            device_type,
            cert_fingerprint: certificate_fingerprint,
            current_time,
        })?;
        self.device_repository.save(&device).await?;
        Ok(RegisterDeviceResponeSchema {
            user: UserSchema::from(user),
            device: DeviceSchema::from(device),
            signed_certificate,
        })
    }
}
