use crate::controller::put_device_request::PutDeviceRequest;
use crate::exception::app_error::AppError;
use crate::model::device::{Device, DeviceStatusCode};
use anyhow::Result;
use chrono::{DateTime, Utc};
use deadpool_redis::{redis::cmd, Pool};
use http::StatusCode;

pub(crate) async fn upsert_device(
    payload: &PutDeviceRequest,
    redis_pool: Pool,
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

pub(crate) async fn get_device(id: String, redis_pool: Pool) -> Result<Device, AppError> {
    let mut redis_conn = redis_pool.get().await?;
    match cmd("GET").arg(&[id]).query_async(&mut redis_conn).await {
        Ok(device_json) => {
            let device_json: String = device_json;
            let device: Device = serde_json::from_str(&device_json)?;
            Ok(device)
        }
        Err(err) => Err(AppError::from_redis_error(err)),
    }
}
