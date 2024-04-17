use axum::handler::Handler;
use axum::Json;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UpsertDeviceRequest {
    id: String,
    name: String,
    lat: f32,
    lng: f32,
    error: f32,
    status_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UpsertDeviceResponse {
    message: String,
    device: Option<Device>,
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

impl DeviceStatusCode {
    fn from_i32(value: i32) -> Result<DeviceStatusCode, String> {
        match value {
            0 => Ok(DeviceStatusCode::NoFire),
            1 => Ok(DeviceStatusCode::Fire),
            _ => Err("Invalid value for DeviceStatusCode".parse().unwrap()),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Device {
    id: String,
    name: String,
    lat: f32,
    lng: f32,
    error: f32,
    status_code: DeviceStatusCode,
    disabled: bool,
    updated_at_ms: i64,
    created_at_ms: i64,
}

pub(crate) async fn create_device(
    Json(payload): Json<UpsertDeviceRequest>,
) -> (StatusCode, Json<UpsertDeviceResponse>) {
    let now: DateTime<Utc> = Utc::now();
    let status_code: Result<DeviceStatusCode, String> =
        DeviceStatusCode::from_i32(payload.status_code);
    if status_code.is_ok() {
        let device = Device {
            id: payload.id,
            name: payload.name,
            lat: payload.lat,
            lng: payload.lng,
            error: payload.error,
            status_code: status_code.unwrap(),
            disabled: false,
            updated_at_ms: now.timestamp(),
            created_at_ms: now.timestamp(),
        };
        let resp = UpsertDeviceResponse {
            message: "success".parse().unwrap(),
            device: Some(device),
        };
        (StatusCode::OK, Json(resp))
    } else {
        let resp = UpsertDeviceResponse {
            message: status_code.err().unwrap_or("No message".parse().unwrap()),
            device: None,
        };
        (StatusCode::OK, Json(resp))
    }
}
