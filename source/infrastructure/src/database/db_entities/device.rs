use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use ulid::Ulid;

use crate::database::enums::DeviceTypeDb;

#[derive(FromRow, Debug)]
pub struct DeviceDb {
    pub id: Ulid,
    pub user_id: Ulid,
    pub name: String,
    pub device_type: DeviceTypeDb,
    pub cert_fingerprint: String,
    pub registered_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}
