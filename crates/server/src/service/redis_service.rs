use crate::exception::app_error::AppError;
use anyhow::Result;
use deadpool_redis::redis::cmd;
use deadpool_redis::{Config, Pool, Runtime};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::env;

#[derive(Clone)]
pub(crate) struct RedisService {
    pool: Pool,
}

impl RedisService {
    pub(crate) async fn get_by_id<T>(&self, id: String) -> Result<T, AppError>
    where
        T: DeserializeOwned,
    {
        let mut redis_conn = self.pool.get().await?;
        match cmd("GET")
            .arg(&[id])
            .query_async::<_, String>(&mut redis_conn)
            .await
        {
            Ok(item_json) => Ok(serde_json::from_str(&item_json)?),
            Err(err) => Err(AppError::from_redis_error(err)),
        }
    }
}

impl Default for RedisService {
    fn default() -> Self {
        let redis_cfg = Config::from_url(
            env::var("REDIS__URL").unwrap_or("redis://localhost:10001".parse().unwrap()),
        );
        Self {
            pool: redis_cfg
                .create_pool(Some(Runtime::Tokio1))
                .expect("Unable to create Redis pool"),
        }
    }
}
