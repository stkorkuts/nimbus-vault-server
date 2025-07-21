pub mod errors;
pub mod register;

pub struct UserSchema {
    pub id: String,
    pub username: String,
    pub encrypted_master_key: String,
}
