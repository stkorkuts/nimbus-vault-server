use std::error::Error;

use crate::{
    database::{Database, DatabaseSettings},
    services::DefaultUserRepository,
};

pub struct InfrastructureConfigurator {
    database: Database,
}

impl InfrastructureConfigurator {
    pub async fn init(database_settings: DatabaseSettings) -> Result<Self, Box<dyn Error>> {
        let database = Database::init(database_settings).await?;
        Ok(Self { database })
    }

    pub fn configure_user_repository(&self) -> DefaultUserRepository {
        DefaultUserRepository::init(&self.database)
    }
}
