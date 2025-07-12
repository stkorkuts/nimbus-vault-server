use chrono::{DateTime, Utc};
use ulid::Ulid;

use crate::enums::DeviceType;

pub struct NewDeviceSpecification {
    pub user_id: Ulid,
    pub name: String,
    pub device_type: DeviceType,
    pub cert_fingerprint: String,
    pub current_time: DateTime<Utc>,
}

pub struct RestoreDeviceSpecification {
    pub id: Ulid,
    pub user_id: Ulid,
    pub name: String,
    pub device_type: DeviceType,
    pub cert_fingerprint: String,
    pub registered_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}
