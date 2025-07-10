use nimbus_vault_server_domain::User;
use ulid::Ulid;

pub trait UserRepository {
    fn get_by_id(&self, id: Ulid) -> User;
    fn save(&self, user: User);
}
