use nimbus_vault_server_domain::enums::DeviceType;

use crate::database::enums::DeviceTypeDb;

impl DeviceTypeDb {
    pub fn to_domain(&self) -> DeviceType {
        match self {
            DeviceTypeDb::MacOS => DeviceType::MacOS,
            DeviceTypeDb::Linux => DeviceType::Linux,
            DeviceTypeDb::Windows => DeviceType::Windows,
        }
    }

    pub fn from_domain(device_type: DeviceType) -> Self {
        match device_type {
            DeviceType::MacOS => DeviceTypeDb::MacOS,
            DeviceType::Linux => DeviceTypeDb::Linux,
            DeviceType::Windows => DeviceTypeDb::Windows,
        }
    }
}
