use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum UserError {
    InvalidUsername { error_message: String },
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUsername { error_message } => {
                write!(f, "Invalid username. Error message: {}", error_message)
            }
        }
    }
}

impl Error for UserError {}
