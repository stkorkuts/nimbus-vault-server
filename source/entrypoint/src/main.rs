use nimbus_vault_server_application::use_cases::builder::ApplicationUseCasesBuilder;
use nimbus_vault_server_infrastructure::{
    database::{Database, DatabaseSettings},
    services::repositories::DefaultUserRepository,
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

    let user_repository = Arc::new(DefaultUserRepository::new(database));

    let use_cases = ApplicationUseCasesBuilder::new()
        .with_register_user(user_repository)
        .build()?;

    Ok(())
}
