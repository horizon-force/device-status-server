use serde::{Deserialize, Serialize};

// TODO: create builder rather than making all fields public
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub(crate) enum DeviceStatusCode {
    #[default]
    NoFire = 0,
    Fire = 1,
}
