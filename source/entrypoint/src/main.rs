use std::sync::Arc;

use nimbus_vault_server_application::use_cases::builder::ApplicationUseCasesBuilder;
use nimbus_vault_server_infrastructure::{
    database::{Database, DatabaseSettings},
    services::{
        repositories::{
            device_repository::DefaultDeviceRepository, user_repository::DefaultUserRepository,
        },
        time::DefaultTimeService,
    },
};
use nimbus_vault_server_shared::environment::DATABASE_URL_VAR_NAME;

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

    let time_service = Arc::new(DefaultTimeService::new());

    let use_cases = ApplicationUseCasesBuilder::new()
        .with_user_repository(user_repository.clone())
        .with_time_service(time_service.clone())
        .build()?;

    Ok(())
}
