use nimbus_vault_server_application::{ApplicationServicesBuilder, ApplicationUseCases};
use nimbus_vault_server_infrastructure::{DatabaseSettings, InfrastructureConfigurator};
use nimbus_vault_server_shared::DATABASE_URL_VAR_NAME;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let infrastructure_configurator = InfrastructureConfigurator::init(DatabaseSettings::new(
        dotenvy::var(DATABASE_URL_VAR_NAME).unwrap(),
    ))
    .await?;

    let user_repository = infrastructure_configurator.configure_user_repository();

    let services = ApplicationServicesBuilder::init()
        .with_user_repository(Box::new(user_repository))
        .build()?;

    let use_cases = ApplicationUseCases::init(&services);

    Ok(())
}
