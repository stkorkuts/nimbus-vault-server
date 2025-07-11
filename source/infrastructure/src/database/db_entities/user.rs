use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct UserDb {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub e2e_key_hash: String,
    pub encrypted_master_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
