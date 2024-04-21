use crate::exception::app_error::AppError;
use anyhow::Result;
use chrono::{DateTime, Utc};
use deadpool_redis::redis::cmd;
use deadpool_redis::{Config, Pool, Runtime};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;

#[derive(Clone)]
pub(crate) struct RedisService {
    pool: Pool,
}

impl RedisService {
    pub(crate) async fn put_with_id<T>(&self, id: String, item: T) -> Result<T, AppError>
    where
        T: Serialize,
    {
        let mut redis_conn = self.pool.get().await?;
        cmd("SET")
            .arg(&[id, serde_json::to_string(&item)?])
            .query_async::<_, ()>(&mut redis_conn)
            .await?;
        Ok(item)
    }

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

    pub(crate) async fn get_all<T>(&self) -> Result<Vec<T>, AppError>
    where
        T: DeserializeOwned,
    {
        let mut result = Vec::new();
        let mut redis_conn = self.pool.get().await?;
        let keys: Vec<String> = cmd("KEYS").arg("*").query_async(&mut redis_conn).await?;
        log::info!("Getting all values from Redis...");
        let now: DateTime<Utc> = Utc::now();
        for key in keys {
            // TODO: make this faster by using threading for the get_by_id() call
            result.push(self.get_by_id(key).await?)
        }
        let time_delta = Utc::now() - now;
        log::info!(
            "Took {}ms to get all values from Redis",
            time_delta.num_milliseconds()
        );
        Ok(result)
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
