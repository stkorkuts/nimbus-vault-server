pub mod errors;

use async_trait::async_trait;
use nimbus_vault_server_domain::entities::device::Device;

use crate::services::repositories::device_repository::errors::DeviceRepositoryError;

#[async_trait]
pub trait DeviceRepository: Send + Sync {
    async fn get_by_fingerprint(
        &self,
        cert_fingerprint: &str,
    ) -> Result<Option<Device>, DeviceRepositoryError>;
    async fn save(&self, device: &Device) -> Result<(), DeviceRepositoryError>;
}
