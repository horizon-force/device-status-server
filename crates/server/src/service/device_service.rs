use crate::config::app_state::AppState;
use crate::controller::put_device_request::PutDeviceRequest;
use crate::exception::app_error::AppError;
use crate::model::device::{Device, DeviceStatusCode};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub(crate) async fn put_device(
    payload: &PutDeviceRequest,
    app_state: AppState,
) -> Result<Device, AppError> {
    let now: DateTime<Utc> = Utc::now();
    let id: String = payload.id().parse()?;
    let device = Device {
        id: id.clone(),
        name: payload.name().parse()?,
        lat: payload.lat(),
        lng: payload.lng(),
        error: payload.error(),
        status_code: payload.status_code(),
        disabled: payload.disabled(),
        updated_at_ms: now.timestamp(),
        created_at_ms: now.timestamp(),
    };
    // immediately update in-memory device cache if FIRE is detected for device
    if let DeviceStatusCode::Fire = device.status_code {
        log::info!("Detected fire for device {:?}", device);
        log::info!("TODO: send MQTT message to all clients that fire was detected");
        let guard_attempt = app_state.device_cache.upgrade();
        if let Some(guard) = guard_attempt {
            guard.write().await.insert(id.clone(), device.clone());
        } else {
            log::warn!("Unable to acquire lock on device cache for update");
        }
    }
    app_state
        .redis_service
        .put_with_id::<Device>(id, device)
        .await
}

pub(crate) async fn get_device(id: String, app_state: AppState) -> Result<Device, AppError> {
    app_state.redis_service.get_by_id::<Device>(id).await
}

pub(crate) async fn get_devices(app_state: AppState) -> Result<HashMap<String, Device>, AppError> {
    let device_cache = app_state
        .device_cache
        .upgrade()
        .expect("Unable to acquire lock on device cache for reading");
    let device_cache_read_guard = device_cache.read().await;
    Ok(device_cache_read_guard.clone())
}
