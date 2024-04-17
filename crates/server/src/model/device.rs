use anyhow::Result;
use serde::{Deserialize, Serialize};

// TODO: create builder rather than making all fields public
#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Device {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) lat: f32,
    pub(crate) lng: f32,
    pub(crate) error: f32,
    pub(crate) status_code: DeviceStatusCode,
    pub(crate) disabled: bool,
    pub(crate) updated_at_ms: i64,
    pub(crate) created_at_ms: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum DeviceStatusCode {
    NoFire = 0,
    Fire = 1,
}

impl Default for DeviceStatusCode {
    fn default() -> Self {
        DeviceStatusCode::NoFire
    }
}

// TODO: implement actual from / into trait
impl DeviceStatusCode {
    pub(crate) fn from_i32(value: i32) -> Result<DeviceStatusCode, String> {
        match value {
            0 => Ok(DeviceStatusCode::NoFire),
            1 => Ok(DeviceStatusCode::Fire),
            _ => Err("Invalid value for DeviceStatusCode".parse().unwrap()),
        }
    }
}
