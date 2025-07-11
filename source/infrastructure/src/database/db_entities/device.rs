use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

use crate::database::enums::DeviceTypeDb;

#[derive(FromRow, Debug)]
pub struct DeviceDb {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub device_type: DeviceTypeDb,
    pub cert_fingerprint: String,
    pub registered_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
}
