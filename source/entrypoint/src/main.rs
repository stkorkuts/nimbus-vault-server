use nimbus_vault_server_infrastructure::{
    InfrastructureConfiguration, InfrastructureSettings, webapi,
};
use nimbus_vault_server_shared::DATABASE_URL_VAR_NAME;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = InfrastructureConfiguration::new(InfrastructureSettings::new(
        dotenvy::var(DATABASE_URL_VAR_NAME).unwrap(),
    ));

    webapi::run_server("127.0.0.1:3000").await?;

    Ok(())
}
