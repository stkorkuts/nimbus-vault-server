pub mod builder;
mod user;

use std::error::Error;

use user::RegisterUserUseCase;

pub struct ApplicationUseCases {
    register_user: RegisterUserUseCase,
}

impl ApplicationUseCases {
    pub async fn register_user(&self) -> Result<(), Box<dyn Error>> {
        self.register_user.execute().await
    }
}
