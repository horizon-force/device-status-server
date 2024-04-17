use crate::controller::device_controller::{PutDeviceRequest, PutDeviceResponse};
use crate::model::device::{Device, DeviceStatusCode};
use anyhow::Result;
use axum::Json;
use chrono::{DateTime, Utc};
use http::StatusCode;

pub(crate) async fn upsert_device(payload: &PutDeviceRequest) -> Result<Device, String> {
    let now: DateTime<Utc> = Utc::now();
    let status_code = DeviceStatusCode::from_i32(payload.status_code())?;
    Ok(Device {
        id: payload.id().parse().unwrap(),
        name: payload.name().parse().unwrap(),
        lat: payload.lat(),
        lng: payload.lng(),
        error: payload.error(),
        status_code,
        disabled: false,
        updated_at_ms: now.timestamp(),
        created_at_ms: now.timestamp(),
    })
}
