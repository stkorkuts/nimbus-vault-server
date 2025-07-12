use nimbus_vault_server_application::use_cases::builder::ApplicationUseCasesBuilder;
use nimbus_vault_server_infrastructure::{
    database::{Database, DatabaseSettings},
    services::repositories::{DefaultDeviceRepository, DefaultUserRepository},
};
use nimbus_vault_server_shared::DATABASE_URL_VAR_NAME;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = Arc::new(
        Database::new(DatabaseSettings::new(
            dotenvy::var(DATABASE_URL_VAR_NAME).unwrap(),
        ))
        .await?,
    );

    let user_repository = Arc::new(DefaultUserRepository::new(database.clone()));
    let device_repository = Arc::new(DefaultDeviceRepository::new(database.clone()));

    let use_cases = ApplicationUseCasesBuilder::new()
        .with_register_user(user_repository.clone())
        .build()?;

    Ok(())
}
