use thiserror::Error;

use crate::entities::user::value_objects::errors::{UserPasswordError, UsernameError};

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Username(#[from] UsernameError),
    #[error(transparent)]
    Password(#[from] UserPasswordError),
}
