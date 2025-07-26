use nimbus_vault_server_application::use_cases::user::UserSchema;

include!(concat!(env!("OUT_DIR"), "/nimbus_vault.auth.rs"));

impl From<UserSchema> for UserProto {
    fn from(value: UserSchema) -> Self {
        UserProto {
            id: value.id,
            username: value.username,
            encrypted_master_key: value.encrypted_master_key,
        }
    }
}
