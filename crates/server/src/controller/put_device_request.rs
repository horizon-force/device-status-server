use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PutDeviceRequest {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) lat: f32,
    pub(crate) lng: f32,
    pub(crate) error: f32,
    pub(crate) status_code: i32,
}

impl PutDeviceRequest {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn lat(&self) -> f32 {
        self.lat
    }
    pub fn lng(&self) -> f32 {
        self.lng
    }
    pub fn error(&self) -> f32 {
        self.error
    }
    pub fn status_code(&self) -> i32 {
        self.status_code
    }
}
