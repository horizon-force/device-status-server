use crate::exception::app_error::AppError;
use crate::model::device::Device;
use crate::service::device_service::upsert_device;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PutDeviceRequest {
    id: String,
    name: String,
    lat: f32,
    lng: f32,
    error: f32,
    status_code: i32,
}

// TODO: use macro to implement getters
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PutDeviceResponse {
    message: String,
    device: Option<Device>,
}

pub(crate) async fn put_device(
    Json(payload): Json<PutDeviceRequest>,
) -> Result<Json<PutDeviceResponse>, AppError> {
    Ok(Json(PutDeviceResponse {
        message: "success".parse().unwrap(),
        device: Some(upsert_device(&payload).await?),
    }))
}
