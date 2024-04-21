use crate::config::app_state::AppState;
use crate::controller::get_device_response::{GetDeviceResponse, GetDevicesResponse};
use crate::controller::put_device_request::PutDeviceRequest;
use crate::controller::put_device_response::PutDeviceResponse;
use crate::exception::app_error::AppError;
use crate::service::device_service;
use axum::extract::{Path, State};
use axum::Json;

pub(crate) async fn put_device(
    State(app_state): State<AppState>,
    Json(payload): Json<PutDeviceRequest>,
) -> Result<Json<PutDeviceResponse>, AppError> {
    Ok(Json(PutDeviceResponse {
        message: "success".parse()?,
        device: Some(device_service::put_device(&payload, app_state).await?),
    }))
}

pub(crate) async fn get_device(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetDeviceResponse>, AppError> {
    Ok(Json(GetDeviceResponse {
        message: "success".parse()?,
        device: Some(device_service::get_device(id, app_state).await?),
    }))
}

pub(crate) async fn get_devices(
    State(app_state): State<AppState>,
) -> Result<Json<GetDevicesResponse>, AppError> {
    Ok(Json(GetDevicesResponse {
        message: "success".parse()?,
        devices: Some(device_service::get_devices(app_state).await?),
    }))
}
