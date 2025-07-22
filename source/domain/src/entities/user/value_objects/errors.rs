use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserPasswordError {
    #[error("User password has invalid length, should be from {min} to {max}")]
    InvalidLength { min: i32, max: i32 },
    #[error("User password is weak. Error: {error_message}")]
    WeakPassword { error_message: String },
}

#[derive(Debug, Error)]
pub enum UsernameError {
    #[error("Username has invalid length, should be from {min} to {max}")]
    InvalidLength { min: i32, max: i32 },
    #[error("User password contains invalid characters. Error: {error_message}")]
    InvalidCharacters { error_message: String },
}
