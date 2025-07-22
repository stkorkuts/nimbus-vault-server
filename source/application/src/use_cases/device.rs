use nimbus_vault_server_domain::entities::device::Device;

pub mod errors;
pub mod register;

pub struct DeviceSchema {
    pub id: String,
    pub name: String,
}

impl From<Device> for DeviceSchema {
    fn from(value: Device) -> Self {
        DeviceSchema {
            id: value.id().to_string(),
            name: value.name().to_owned(),
        }
    }
}
