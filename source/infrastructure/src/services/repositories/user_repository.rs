pub struct UserRepository {}

impl nimbus_vault_server_application::UserRepository for UserRepository {
    fn get_by_id(&self, id: ulid::Ulid) -> nimbus_vault_server_domain::User {
        todo!()
    }
}
