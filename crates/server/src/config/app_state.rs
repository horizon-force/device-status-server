use crate::service::redis_service::RedisService;
use async_std::sync::RwLock;
use async_std::sync::Weak;
use deadpool_redis::Pool;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) redis_pool: Pool,
    pub(crate) device_cache: Weak<RwLock<HashMap<String, String>>>,
    pub(crate) redis_service: RedisService,
}
