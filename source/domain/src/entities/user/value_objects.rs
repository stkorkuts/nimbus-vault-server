use crate::entities::user::value_objects::errors::{UserPasswordError, UsernameError};

pub mod errors;

pub struct UserPassword {
    value: String,
}

pub struct Username {
    value: String,
}

impl UserPassword {
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn new(password: &str) -> Result<Self, UserPasswordError> {
        Self::validate(password)?;
        Ok(Self {
            value: password.to_owned(),
        })
    }

    fn validate(password: &str) -> Result<Self, UserPasswordError> {
        todo!()
    }
}

impl Username {
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn new(username: &str) -> Result<Self, UsernameError> {
        Self::validate(username)?;
        Ok(Self {
            value: username.to_owned(),
        })
    }

    fn validate(username: &str) -> Result<Self, UsernameError> {
        todo!()
    }
}
