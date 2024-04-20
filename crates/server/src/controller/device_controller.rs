use crate::controller::get_device_response::GetDeviceResponse;
use crate::controller::put_device_request::PutDeviceRequest;
use crate::controller::put_device_response::PutDeviceResponse;
use crate::exception::app_error::AppError;
use crate::service::device_service;
use axum::extract::{Path, State};
use axum::Json;
use deadpool_redis::Pool;

pub(crate) async fn put_device(
    State(redis_pool): State<Pool>,
    Json(payload): Json<PutDeviceRequest>,
) -> Result<Json<PutDeviceResponse>, AppError> {
    Ok(Json(PutDeviceResponse {
        message: "success".parse()?,
        device: Some(device_service::upsert_device(&payload, redis_pool).await?),
    }))
}

pub(crate) async fn get_device(
    State(redis_pool): State<Pool>,
    Path(id): Path<String>,
) -> Result<Json<GetDeviceResponse>, AppError> {
    Ok(Json(GetDeviceResponse {
        message: "success".parse()?,
        device: Some(device_service::get_device(id, redis_pool).await?),
    }))
}
