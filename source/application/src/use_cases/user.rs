use nimbus_vault_server_domain::entities::user::User;

pub mod errors;
pub mod register;

pub struct UserSchema {
    pub id: String,
    pub username: String,
    pub encrypted_master_key: String,
}

impl From<User> for UserSchema {
    fn from(value: User) -> Self {
        UserSchema {
            id: value.id().to_string(),
            username: value.username().to_owned(),
            encrypted_master_key: value.encrypted_master_key().to_owned(),
        }
    }
}
