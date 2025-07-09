use std::error::Error;

use crate::services::ApplicationServices;

mod user;

pub use user::RegisterUserUseCase;

pub struct ApplicationUseCases<'a> {
    register_user: RegisterUserUseCase<'a>,
}

impl<'a> ApplicationUseCases<'a> {
    pub fn init(services: &'a ApplicationServices) -> Self {
        ApplicationUseCases {
            register_user: RegisterUserUseCase::init(services.user_repository()),
        }
    }

    pub fn register_user(&self) -> Result<(), Box<dyn Error>> {
        self.register_user.execute()
    }
}
