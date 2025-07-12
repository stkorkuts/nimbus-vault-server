use std::error::Error;

use async_trait::async_trait;
use nimbus_vault_server_domain::entities::Device;

#[async_trait]
pub trait DeviceRepository {
    async fn get_by_fingerprint(
        &self,
        cert_fingerprint: &str,
    ) -> Result<Option<Device>, Box<dyn Error>>;
    async fn save(&self, device: Device) -> Result<(), Box<dyn Error>>;
}
