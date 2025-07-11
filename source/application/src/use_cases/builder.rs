use std::{error::Error, sync::Arc};

use crate::{
    services::repositories::UserRepository,
    use_cases::{ApplicationUseCases, user::RegisterUserUseCase},
};

pub struct ApplicationUseCasesBuilder {
    register_user: Option<RegisterUserUseCase>,
}

impl ApplicationUseCasesBuilder {
    pub fn new() -> Self {
        ApplicationUseCasesBuilder {
            register_user: None,
        }
    }

    pub fn with_register_user(&mut self, user_repository: Arc<dyn UserRepository>) -> &mut Self {
        self.register_user = Some(RegisterUserUseCase::new(user_repository));
        self
    }

    pub fn build(&self) -> Result<ApplicationUseCases, Box<dyn Error>> {
        todo!()
    }
}
