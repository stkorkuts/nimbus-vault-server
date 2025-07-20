use std::{error::Error, sync::Arc};

use nimbus_vault_server_domain::enums::DeviceType;

use crate::{
    services::{
        auth::AuthService,
        crypto::CryptoService,
        repositories::{device_repository::DeviceRepository, user_repository::UserRepository},
        time::TimeService,
    },
    use_cases::{device::DeviceSchema, user::UserSchema},
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
    ) -> Result<RegisterDeviceResponeSchema, Box<dyn Error>> {
        // get user from repo if username exists
        // generate password hash to check if it is valid
        // validate if e2e key hash is the same
        // sign certificate and generate a fingerprint
        // save device to the database
        // return user info + device info + signed certificate
        todo!()
    }
}
