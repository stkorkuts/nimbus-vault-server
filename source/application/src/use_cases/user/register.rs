use std::error::Error;

use crate::UserRepository;

pub struct RegisterUserUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> RegisterUserUseCase<'a> {
    pub fn init(user_repository: &'a dyn UserRepository) -> Self {
        Self { user_repository }
    }

    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
