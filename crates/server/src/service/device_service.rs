use crate::controller::device_controller::PutDeviceRequest;
use crate::exception::app_error::AppError;
use crate::model::device::{Device, DeviceStatusCode};
use anyhow::Result;
use chrono::{DateTime, Utc};
use http::StatusCode;

pub(crate) async fn upsert_device(payload: &PutDeviceRequest) -> Result<Device, AppError> {
    let now: DateTime<Utc> = Utc::now();
    match DeviceStatusCode::from_i32(payload.status_code()) {
        Ok(status_code) => Ok(Device {
            id: payload.id().parse().unwrap(),
            name: payload.name().parse().unwrap(),
            lat: payload.lat(),
            lng: payload.lng(),
            error: payload.error(),
            status_code,
            disabled: false,
            updated_at_ms: now.timestamp(),
            created_at_ms: now.timestamp(),
        }),
        Err(err) => Err(AppError::from(err, StatusCode::BAD_REQUEST)),
    }
}
