use std::{error::Error, sync::Arc};

use crate::services::repositories::UserRepository;

pub struct RegisterUserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl RegisterUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
