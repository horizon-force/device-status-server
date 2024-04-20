use crate::model::device::Device;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PutDeviceResponse {
    pub(crate) message: String,
    pub(crate) device: Option<Device>,
}
