use nimbus_vault_server_application::{ApplicationEnvironment, ApplicationServicesBuilder};
use nimbus_vault_server_infrastructure::{InfrastructureConfiguration, InfrastructureSettings};
use nimbus_vault_server_shared::DATABASE_URL_VAR_NAME;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = InfrastructureConfiguration::new(InfrastructureSettings::new(
        dotenvy::var(DATABASE_URL_VAR_NAME).unwrap(),
    ));

    let services = ApplicationServicesBuilder::init()
        .with_user_repository(Box::new(config.configure_user_repository()))
        .build()?;

    let app_env = ApplicationEnvironment::init(services);

    Ok(())
}
