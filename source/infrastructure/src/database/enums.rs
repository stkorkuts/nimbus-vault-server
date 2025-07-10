use sqlx::prelude::Type;

#[derive(Type, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "device_type")]
pub enum DeviceTypeDb {
    Linux,
    Windows,
    MacOS,
}
