pub mod errors;

use std::{error::Error, sync::Arc};

use nimbus_vault_server_domain::enums::DeviceType;

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
    pub certificate_sign_response: Vec<u8>,
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
            .get_by_username(username)
            .await
            .map_err(RepositoryError::from)
            .map_err(ServiceError::from)?
            .ok_or(RegisterDeviceUseCaseError::UserNotFound)?;
        let password_hash = self.crypto_service.get_password_hash(password).await?;

        // generate password hash to check if it is valid
        // validate if e2e key hash is the same
        // sign certificate and generate a fingerprint
        // save device to the database
        // return user info + device info + signed certificate
        todo!()
    }
}
