use crate::config::app_state::AppState;
use crate::controller::get_device_response::{GetDeviceResponse, GetDevicesResponse};
use crate::controller::put_device_request::PutDeviceRequest;
use crate::controller::put_device_response::PutDeviceResponse;
use crate::exception::app_error::AppError;
use crate::service::device_service;
use axum::extract::{Path, State};
use axum::Json;
use deadpool_redis::Pool;
use std::collections::HashMap;

pub(crate) async fn put_device(
    State(app_state): State<AppState>,
    Json(payload): Json<PutDeviceRequest>,
) -> Result<Json<PutDeviceResponse>, AppError> {
    Ok(Json(PutDeviceResponse {
        message: "success".parse()?,
        device: Some(device_service::upsert_device(&payload, app_state.redis_pool).await?),
    }))
}

pub(crate) async fn get_device(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetDeviceResponse>, AppError> {
    Ok(Json(GetDeviceResponse {
        message: "success".parse()?,
        device: Some(device_service::get_device(id, app_state.redis_pool).await?),
    }))
}

pub(crate) async fn get_devices(
    State(app_state): State<AppState>,
) -> Result<Json<GetDevicesResponse>, AppError> {
    let device_cache_read_guard = app_state
        .device_cache
        .upgrade()
        .expect("Unable to acquire lock on device cache for reading");
    let device_cache = device_cache_read_guard.read().await;
    Ok(Json(GetDevicesResponse {
        message: "success".parse()?,
        devices: Some(device_cache.clone()),
    }))
}
