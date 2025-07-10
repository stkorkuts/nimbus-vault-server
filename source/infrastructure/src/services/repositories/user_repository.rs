use nimbus_vault_server_application::UserRepository;
use nimbus_vault_server_domain::User;

use crate::database::{self, Database};

pub struct DefaultUserRepository<'a> {
    database: &'a Database,
}

impl<'a> DefaultUserRepository<'a> {
    pub fn init(database: &'a Database) -> Self {
        Self { database }
    }
}

impl<'a> UserRepository for DefaultUserRepository<'a> {
    fn get_by_id(&self, id: ulid::Ulid) -> nimbus_vault_server_domain::User {
        todo!()
    }

    fn save(&self, user: User) {
        todo!()
    }
}
