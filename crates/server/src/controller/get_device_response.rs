use crate::model::device::Device;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GetDeviceResponse {
    pub(crate) message: String,
    pub(crate) device: Option<Device>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GetDevicesResponse {
    pub(crate) message: String,
    pub(crate) devices: Option<HashMap<String, Device>>,
}
