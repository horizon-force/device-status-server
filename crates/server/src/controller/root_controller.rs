use crate::exception::app_error::AppError;

pub(crate) async fn root() -> Result<String, AppError> {
    Ok("hello from device-status-server".parse()?)
}
