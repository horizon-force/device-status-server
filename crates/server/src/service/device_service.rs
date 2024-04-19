use crate::controller::device_controller::PutDeviceRequest;
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
            let id: String = payload.id().parse().unwrap();
            let device = Device {
                id: id.clone(),
                name: payload.name().parse().unwrap(),
                lat: payload.lat(),
                lng: payload.lng(),
                error: payload.error(),
                status_code,
                disabled: false,
                updated_at_ms: now.timestamp(),
                created_at_ms: now.timestamp(),
            };
            let mut redis_conn = redis_pool.get().await.unwrap();
            cmd("SET")
                .arg(&[id, serde_json::to_string(&device)?])
                .query_async::<_, ()>(&mut redis_conn)
                .await
                .unwrap();
            Ok(device)
        }
        Err(err) => Err(AppError::from(err, StatusCode::BAD_REQUEST)),
    }
}
