use std::{error::Error, fmt::Display};

#[derive(Debug)]

pub enum DeviceError {
    InvalidName { error_message: String },
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName { error_message } => {
                write!(f, "Invalid device name. Error: {}", error_message)
            }
        }
    }
}

impl Error for DeviceError {}
