use crate::model::device::{Device, DeviceStatusCode};
use crate::service::device_service::upsert_device;
use axum::handler::Handler;
use axum::Json;
use chrono::{DateTime, Utc};
use http::StatusCode;
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

// TODO: use annotation to implement getters
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
) -> (StatusCode, Json<PutDeviceResponse>) {
    let result = upsert_device(&payload).await;
    if let Ok(device) = result {
        (
            StatusCode::OK,
            Json(PutDeviceResponse {
                message: "success".parse().unwrap(),
                device: Some(device),
            }),
        )
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(PutDeviceResponse {
                message: result.err().unwrap_or("internal error".parse().unwrap()),
                device: None,
            }),
        )
    }
}
