use chrono::{DateTime, Utc};
use ulid::Ulid;

pub struct NewUserSpecification {
    pub username: String,
    pub password_hash: String,
    pub e2e_key_hash: String,
    pub encrypted_master_key: String,
    pub current_time: DateTime<Utc>,
}

pub struct RestoreUserSpecification {
    pub id: Ulid,
    pub username: String,
    pub password_hash: String,
    pub e2e_key_hash: String,
    pub encrypted_master_key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
