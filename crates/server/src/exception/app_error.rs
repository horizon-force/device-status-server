use axum::response::{IntoResponse, Response};
use deadpool_redis::redis::RedisError;
use http::StatusCode;

pub(crate) struct AppError(pub(crate) anyhow::Error, pub(crate) StatusCode);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.1, format!("Something went wrong: {}", self.0)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl AppError {
    pub(crate) fn from(err: anyhow::Error, status: StatusCode) -> Self {
        Self(err, status)
    }

    pub(crate) fn from_redis_error(err: RedisError) -> Self {
        let is_internal_error = err.is_timeout()
            || err.is_cluster_error()
            || err.is_connection_dropped()
            || err.is_connection_refusal()
            || err.is_io_error()
            || err.is_unrecoverable_error();
        let status_code = if is_internal_error {
            StatusCode::INTERNAL_SERVER_ERROR
        } else {
            StatusCode::BAD_REQUEST
        };
        Self(err.into(), status_code)
    }
}
