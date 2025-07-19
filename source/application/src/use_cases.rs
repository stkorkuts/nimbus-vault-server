pub mod builder;
pub mod user;

use std::error::Error;

use crate::use_cases::user::register::{
    RegisterUserRequestSchema, RegisterUserResponseSchema, RegisterUserUseCase,
};

pub struct ApplicationUseCases {
    register_user: RegisterUserUseCase,
}

impl ApplicationUseCases {
    pub async fn register_user(
        &self,
        request: RegisterUserRequestSchema,
    ) -> Result<RegisterUserResponseSchema, Box<dyn Error>> {
        self.register_user.execute(request).await
    }
}
