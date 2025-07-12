mod specifications;

use std::error::Error;

use chrono::{DateTime, Utc};
use ulid::Ulid;

use crate::enums::DeviceType;

pub use specifications::*;

#[derive(Debug)]
pub struct Device {
    id: Ulid,
    user_id: Ulid,
    name: String,
    device_type: DeviceType,
    cert_fingerprint: String,
    registered_at: DateTime<Utc>,
    last_seen_at: DateTime<Utc>,
    revoked_at: Option<DateTime<Utc>>,
}

impl Device {
    pub fn id(&self) -> Ulid {
        self.id
    }

    pub fn user_id(&self) -> Ulid {
        self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }

    pub fn cert_fingerprint(&self) -> &str {
        &self.cert_fingerprint
    }

    pub fn registered_at(&self) -> DateTime<Utc> {
        self.registered_at
    }

    pub fn last_seen_at(&self) -> DateTime<Utc> {
        self.last_seen_at
    }

    pub fn revoked_at(&self) -> Option<DateTime<Utc>> {
        self.revoked_at
    }

    pub fn new(specs: NewDeviceSpecification) -> Result<Self, Box<dyn Error>> {
        Self::validate(specs.name.as_str())?;
        let id = Ulid::new();
        Ok(Device {
            id,
            user_id: specs.user_id,
            name: specs.name,
            device_type: specs.device_type,
            cert_fingerprint: specs.cert_fingerprint,
            registered_at: specs.current_time,
            last_seen_at: specs.current_time,
            revoked_at: None,
        })
    }

    pub fn restore(specs: RestoreDeviceSpecification) -> Result<Self, Box<dyn Error>> {
        Self::validate(specs.name.as_str())?;
        Ok(Device {
            id: specs.id,
            user_id: specs.user_id,
            name: specs.name,
            device_type: specs.device_type,
            cert_fingerprint: specs.cert_fingerprint,
            registered_at: specs.registered_at,
            last_seen_at: specs.last_seen_at,
            revoked_at: specs.revoked_at,
        })
    }

    fn validate(name: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
