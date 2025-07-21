use thiserror::Error;

use crate::use_cases::user::register::errors::RegisterUserUseCaseError;

#[derive(Debug, Error)]
pub enum UserUseCaseError {
    #[error(transparent)]
    Register(#[from] RegisterUserUseCaseError),
}
