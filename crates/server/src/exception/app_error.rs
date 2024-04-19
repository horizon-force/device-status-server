use axum::response::{IntoResponse, Response};
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
}
