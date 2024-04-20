use async_std::sync::RwLock;
use deadpool_redis::Pool;
use std::rc::Weak;

struct AppConfig {
    redis_pool: Pool,
    device_cache: Weak<RwLock<Vec<String>>>,
}
