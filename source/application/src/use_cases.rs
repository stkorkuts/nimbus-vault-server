pub mod builder;
pub mod device;
pub mod errors;
pub mod user;

use crate::use_cases::user::register::{
    RegisterUserRequestSchema, RegisterUserResponseSchema, RegisterUserUseCase,
    errors::RegisterUserUseCaseError,
};

pub struct ApplicationUseCases {
    register_user: RegisterUserUseCase,
}

impl ApplicationUseCases {
    pub async fn register_user(
        &self,
        request: RegisterUserRequestSchema,
    ) -> Result<RegisterUserResponseSchema, RegisterUserUseCaseError> {
        self.register_user.execute(request).await
    }
}
