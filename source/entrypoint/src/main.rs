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
    webapi::{WebApi, WebApiSettings},
};
use nimbus_vault_server_shared::environment::{BASE_ADDR_VAR_NAME, DATABASE_URL_VAR_NAME};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_settings = DatabaseSettings::new(dotenvy::var(DATABASE_URL_VAR_NAME).unwrap());
    let webapi_settings = WebApiSettings::new(dotenvy::var(BASE_ADDR_VAR_NAME).unwrap());

    let database = Arc::new(Database::new(database_settings).await?);

    let user_repository = Arc::new(DefaultUserRepository::new(database.clone()));
    let device_repository = Arc::new(DefaultDeviceRepository::new(database.clone()));

    let time_service = Arc::new(DefaultTimeService::new());

    let use_cases = Arc::new(
        ApplicationUseCasesBuilder::new()
            .with_user_repository(user_repository.clone())
            .with_time_service(time_service.clone())
            .build()?,
    );

    let webapi = WebApi::new(webapi_settings, use_cases.clone());

    let (webapi_result,) = tokio::join!(webapi.serve());

    webapi_result?;
    Ok(())
}
