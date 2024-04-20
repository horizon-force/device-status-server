use crate::model::device::Device;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GetDeviceResponse {
    pub(crate) message: String,
    pub(crate) device: Option<Device>,
}
