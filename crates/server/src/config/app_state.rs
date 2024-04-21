use crate::model::device::Device;
use crate::service::redis_service::RedisService;
use async_std::sync::RwLock;
use async_std::sync::Weak;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) device_cache: Weak<RwLock<HashMap<String, Device>>>,
    pub(crate) redis_service: RedisService,
}
