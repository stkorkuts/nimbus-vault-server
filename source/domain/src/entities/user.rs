mod specifications;

use std::error::Error;

use chrono::{DateTime, Utc};
use ulid::Ulid;

pub use specifications::*;

#[derive(Debug)]
pub struct User {
    id: Ulid,
    username: String,
    password_hash: String,
    e2e_key_hash: String,
    encrypted_master_key: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn id(&self) -> Ulid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn e2e_key_hash(&self) -> &str {
        &self.e2e_key_hash
    }

    pub fn encrypted_master_key(&self) -> &str {
        &self.encrypted_master_key
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn new(specs: NewUserSpecification) -> Result<Self, Box<dyn Error>> {
        Self::validate(
            specs.username.as_str(),
            specs.password_hash.as_str(),
            specs.e2e_key_hash.as_str(),
            specs.encrypted_master_key.as_str(),
        )?;
        let id = Ulid::new();
        Ok(User {
            id: id,
            username: specs.username,
            password_hash: specs.password_hash,
            e2e_key_hash: specs.e2e_key_hash,
            encrypted_master_key: specs.encrypted_master_key,
            created_at: specs.current_time,
            updated_at: specs.current_time,
        })
    }

    pub fn restore(specs: RestoreUserSpecification) -> Result<Self, Box<dyn Error>> {
        Self::validate(
            specs.username.as_str(),
            specs.password_hash.as_str(),
            specs.e2e_key_hash.as_str(),
            specs.encrypted_master_key.as_str(),
        )?;
        Ok(User {
            id: specs.id,
            username: specs.username,
            password_hash: specs.password_hash,
            e2e_key_hash: specs.e2e_key_hash,
            encrypted_master_key: specs.encrypted_master_key,
            created_at: specs.created_at,
            updated_at: specs.updated_at,
        })
    }

    fn validate(
        username: &str,
        password_hash: &str,
        e2e_key_hash: &str,
        encrypted_master_key: &str,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
