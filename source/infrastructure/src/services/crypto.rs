use std::str::FromStr;

use async_trait::async_trait;
use hex;
use hmac::Hmac;
use nimbus_vault_server_application::services::crypto::{
    CryptoService, errors::CryptoServiceError,
};
use nimbus_vault_server_domain::{
    entities::user::value_objects::UserPassword,
    value_objects::password_hash::PasswordHashAlgorithm,
};
use pbkdf2::pbkdf2;
use rand::Rng;
use sha2::Sha256;

pub struct DefaultCryptoService {}

impl DefaultCryptoService {
    fn hash_pbkdf2_sha256(
        &self,
        password: &str,
        iterations: u32,
        salt: &[u8],
    ) -> Result<[u8; 32], CryptoServiceError> {
        let mut hash = [0u8; 32];
        pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, iterations, &mut hash).map_err(|e| {
            CryptoServiceError::HashingFailed {
                error_message: format!("PBKDF2 error: {}", e),
            }
        })?;
        Ok(hash)
    }

    fn verify_pbkdf2_sha256_parts(
        &self,
        password: &str,
        iterations: u32,
        salt_len: usize,
        salt_hex: &str,
        hash_hex: &str,
    ) -> Result<bool, CryptoServiceError> {
        let salt = hex::decode(salt_hex).map_err(|_| CryptoServiceError::InvalidFormat {
            error_message: "Salt is not valid hex".to_string(),
        })?;
        let hash = hex::decode(hash_hex).map_err(|_| CryptoServiceError::InvalidFormat {
            error_message: "Hash is not valid hex".to_string(),
        })?;
        if salt.len() != salt_len || hash.len() != 32 {
            return Err(CryptoServiceError::InvalidFormat {
                error_message: "Salt or hash has invalid length".to_string(),
            });
        }
        self.verify_pbkdf2_sha256(password, iterations, &salt, &hash)
    }
    fn verify_pbkdf2_sha256(
        &self,
        password: &str,
        iterations: u32,
        salt: &[u8],
        expected_hash: &[u8],
    ) -> Result<bool, CryptoServiceError> {
        let hash = self.hash_pbkdf2_sha256(password, iterations, salt)?;
        Ok(expected_hash == hash)
    }
}

#[async_trait]
impl CryptoService for DefaultCryptoService {
    async fn get_user_password_hash(
        &self,
        password: &UserPassword,
        algorithm: Option<PasswordHashAlgorithm>,
    ) -> Result<String, CryptoServiceError> {
        let password_value = password.value();
        let algorithm = algorithm.unwrap_or(PasswordHashAlgorithm::Pbkdf2Sha256 {
            iterations: 100_000,
            salt_len: 32,
        });
        match algorithm {
            PasswordHashAlgorithm::Pbkdf2Sha256 {
                iterations,
                salt_len,
            } => {
                let mut salt = vec![0u8; salt_len];
                rand::rng().fill(&mut salt[..]);
                let hash = self.hash_pbkdf2_sha256(password_value, iterations, &salt)?;
                let salt_hex = hex::encode(&salt);
                let hash_hex = hex::encode(hash);
                let formatted = format!("${}${}${}", algorithm.to_string(), salt_hex, hash_hex);
                Ok(formatted)
            }
        }
    }

    async fn verify_user_password(
        &self,
        password: &UserPassword,
        stored_hash: &str,
    ) -> Result<bool, CryptoServiceError> {
        let password_value = password.value();
        // Format: $<algorithm>$<parameters...>$<salt_hex?>$<hash_hex>
        let parts: Vec<&str> = stored_hash.split('$').collect();
        if parts.len() < 5 || parts[0] != "" {
            return Err(CryptoServiceError::InvalidFormat {
                error_message: "Hash does not match expected format".to_string(),
            });
        }
        let algo_and_params = &parts[1..parts.len() - 2];
        let salt_hex = parts[parts.len() - 2];
        let hash_hex = parts[parts.len() - 1];
        let algorithm = PasswordHashAlgorithm::from_str(&algo_and_params.join("$"))?;
        match algorithm {
            PasswordHashAlgorithm::Pbkdf2Sha256 {
                iterations,
                salt_len,
            } => self.verify_pbkdf2_sha256_parts(
                password_value,
                iterations,
                salt_len,
                salt_hex,
                hash_hex,
            ),
        }
    }
}
