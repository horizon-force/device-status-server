use crate::config::app_state::AppState;
use crate::controller::put_device_request::PutDeviceRequest;
use crate::exception::app_error::AppError;
use crate::model::device::{Device, DeviceStatusCode};
use anyhow::Result;
use chrono::{DateTime, Utc};

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
        disabled: false,
        updated_at_ms: now.timestamp(),
        created_at_ms: now.timestamp(),
    };
    // immediately update in-memory device cache if FIRE is detected for device
    if let DeviceStatusCode::Fire = device.status_code {
        let json_str = serde_json::to_string(&device)?;
        log::info!("Detected fire for device {json_str}");
        log::info!("TODO: send MQTT message to all clients that fire was detected");
        let guard = app_state
            .device_cache
            .upgrade()
            .expect("Unable to acquire lock on device_cache");
        guard.write().await.insert(id.clone(), json_str);
    }
    app_state
        .redis_service
        .put_with_id::<Device>(id, device)
        .await
}

pub(crate) async fn get_device(id: String, app_state: AppState) -> Result<Device, AppError> {
    app_state.redis_service.get_by_id::<Device>(id).await
}
