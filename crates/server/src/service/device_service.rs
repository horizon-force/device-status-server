use crate::config::app_state::AppState;
use crate::controller::put_device_request::PutDeviceRequest;
use crate::exception::app_error::AppError;
use crate::model::device::{Device, DeviceStatusCode};
use anyhow::Result;
use async_std::sync::RwLock;
use async_std::sync::Weak;
use chrono::{DateTime, Utc};
use deadpool_redis::{redis::cmd, Pool};
use http::StatusCode;
use std::collections::HashMap;

pub(crate) async fn upsert_device(
    payload: &PutDeviceRequest,
    redis_pool: Pool,
    device_cache: Weak<RwLock<HashMap<String, String>>>,
) -> Result<Device, AppError> {
    let now: DateTime<Utc> = Utc::now();
    match DeviceStatusCode::from_i32(payload.status_code()) {
        Ok(status_code) => {
            let id: String = payload.id().parse()?;
            let device = Device {
                id: id.clone(),
                name: payload.name().parse()?,
                lat: payload.lat(),
                lng: payload.lng(),
                error: payload.error(),
                status_code,
                disabled: false,
                updated_at_ms: now.timestamp(),
                created_at_ms: now.timestamp(),
            };
            if let DeviceStatusCode::Fire = device.status_code {
                let json_str = serde_json::to_string(&device)?;
                log::info!("Detected fire for device {json_str}");
                log::info!("TODO: send MQTT message to all clients that fire was detected");
                let guard = device_cache
                    .upgrade()
                    .expect("Unable to acquire lock on device_cache");
                guard.write().await.insert(id.clone(), json_str);
            }
            let mut redis_conn = redis_pool.get().await?;
            match cmd("SET")
                .arg(&[id, serde_json::to_string(&device)?])
                .query_async::<_, ()>(&mut redis_conn)
                .await
            {
                Ok(_res) => Ok(device),
                Err(err) => Err(AppError::from_redis_error(err)),
            }
        }
        Err(err) => Err(AppError::from(err, StatusCode::BAD_REQUEST)),
    }
}

pub(crate) async fn get_device(id: String, app_state: AppState) -> Result<Device, AppError> {
    let device = app_state.redis_service.get_by_id::<Device>(id).await?;
    Ok(device)
}
