pub mod errors;

use std::str::FromStr;

use crate::value_objects::password_hash::errors::PasswordHashAlgorithmError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordHashAlgorithm {
    Pbkdf2Sha256 { iterations: u32, salt_len: usize },
    // Future: Argon2, Bcrypt, etc.
}

impl ToString for PasswordHashAlgorithm {
    fn to_string(&self) -> String {
        match self {
            PasswordHashAlgorithm::Pbkdf2Sha256 {
                iterations,
                salt_len,
            } => {
                format!("pbkdf2-sha256${}${}", iterations, salt_len)
            }
        }
    }
}

impl FromStr for PasswordHashAlgorithm {
    type Err = PasswordHashAlgorithmError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('$').collect();
        match parts.as_slice() {
            ["pbkdf2-sha256", iterations, salt_len] => {
                let iterations = iterations.parse::<u32>().map_err(|_| {
                    PasswordHashAlgorithmError::InvalidFormat {
                        error_message: "Iterations is not a valid number".to_string(),
                    }
                })?;
                let salt_len = salt_len.parse::<usize>().map_err(|_| {
                    PasswordHashAlgorithmError::InvalidFormat {
                        error_message: "Salt length is not a valid number".to_string(),
                    }
                })?;
                Ok(PasswordHashAlgorithm::Pbkdf2Sha256 {
                    iterations,
                    salt_len,
                })
            }
            _ => Err(PasswordHashAlgorithmError::UnsupportedAlgorithm {
                algorithm: s.to_string(),
            }),
        }
    }
}
