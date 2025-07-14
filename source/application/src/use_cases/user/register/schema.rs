use crate::use_cases::user::schema::UserSchema;

pub struct RegisterUserRequestSchema {
    pub username: String,
    pub password: String,
    pub e2e_key_hash: String,
    pub encrypted_master_key: String,
}

pub struct RegisterUserResponseSchema {
    pub user: UserSchema,
}
