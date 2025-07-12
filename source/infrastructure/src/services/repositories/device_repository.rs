use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use nimbus_vault_server_application::services::repositories::DeviceRepository;
use nimbus_vault_server_domain::entities::{Device, RestoreDeviceSpecification};
use ulid::Ulid;

use crate::database::{Database, db_entities::DeviceDb, enums::DeviceTypeDb};

pub struct DefaultDeviceRepository {
    database: Arc<Database>,
}

impl DefaultDeviceRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    fn to_db_entity(&self, device: Device) -> DeviceDb {
        DeviceDb {
            id: device.id().to_string(),
            user_id: device.user_id().to_string(),
            name: device.name().to_owned(),
            device_type: DeviceTypeDb::from_domain(device.device_type()),
            cert_fingerprint: device.cert_fingerprint().to_owned(),
            registered_at: device.registered_at(),
            last_seen_at: device.last_seen_at(),
            revoked_at: device.revoked_at(),
        }
    }

    fn to_domain_entity(&self, device_db: DeviceDb) -> Result<Device, Box<dyn Error>> {
        Device::restore(RestoreDeviceSpecification {
            id: Ulid::from_string(device_db.id.as_str())?,
            user_id: Ulid::from_string(device_db.user_id.as_str())?,
            name: device_db.name,
            device_type: device_db.device_type.to_domain(),
            cert_fingerprint: device_db.cert_fingerprint,
            registered_at: device_db.registered_at,
            last_seen_at: device_db.last_seen_at,
            revoked_at: device_db.revoked_at,
        })
    }
}

#[async_trait]
impl DeviceRepository for DefaultDeviceRepository {
    async fn get_by_fingerprint(
        &self,
        cert_fingerprint: &str,
    ) -> Result<Option<Device>, Box<dyn Error>> {
        sqlx::query_as!(
            DeviceDb,
            r#"
            select id, user_id, name, device_type as "device_type: DeviceTypeDb", cert_fingerprint, registered_at, last_seen_at, revoked_at
            from devices
            where cert_fingerprint = $1
            "#,
            cert_fingerprint
        )
        .fetch_optional(self.database.pool())
        .await?
        .map(|device_db| self.to_domain_entity(device_db))
        .transpose()
    }
    async fn save(&self, device: Device) -> Result<(), Box<dyn Error>> {
        let device_db = self.to_db_entity(device);
        sqlx::query!(
            r#"
            insert into devices
            (id, user_id, name, device_type, cert_fingerprint, registered_at, last_seen_at, revoked_at)
            values 
            ($1, $2, $3, $4, $5, $6, $7, $8)
            on conflict(id) do update
                set last_seen_at = excluded.last_seen_at,
                    revoked_at = excluded.revoked_at
            "#,
            device_db.id,
            device_db.user_id,
            device_db.name,
            device_db.device_type as DeviceTypeDb,
            device_db.cert_fingerprint,
            device_db.registered_at,
            device_db.last_seen_at,
            device_db.revoked_at
        )
        .execute(self.database.pool())
        .await?;
        Ok(())
    }
}
