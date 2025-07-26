pub mod builder;
pub mod device;
pub mod errors;
pub mod user;

use crate::use_cases::{
    device::register::{
        RegisterDeviceRequestSchema, RegisterDeviceResponeSchema, RegisterDeviceUseCase,
        errors::RegisterDeviceUseCaseError,
    },
    user::register::{
        RegisterUserResponseSchema, RegisterUserUseCase, SignUpRequestSchema,
        errors::RegisterUserUseCaseError,
    },
};

pub struct ApplicationUseCases {
    register_user: RegisterUserUseCase,
    register_device: RegisterDeviceUseCase,
}

impl ApplicationUseCases {
    pub async fn register_user(
        &self,
        request: SignUpRequestSchema,
    ) -> Result<RegisterUserResponseSchema, RegisterUserUseCaseError> {
        self.register_user.execute(request).await
    }

    pub async fn register_device(
        &self,
        request: RegisterDeviceRequestSchema,
    ) -> Result<RegisterDeviceResponeSchema, RegisterDeviceUseCaseError> {
        self.register_device.execute(request).await
    }
}
