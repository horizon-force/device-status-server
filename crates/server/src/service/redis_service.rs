use crate::exception::app_error::AppError;
use anyhow::Result;
use deadpool_redis::redis::cmd;
use deadpool_redis::{Config, Pool, Runtime};
use futures::future;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use tokio::sync::mpsc::{channel, Sender};

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
        T: DeserializeOwned + Send + 'static,
    {
        let mut redis_conn = self.pool.get().await?;
        let keys: Vec<String> = cmd("KEYS").arg("*").query_async(&mut redis_conn).await?;
        let (sender, mut receiver) = channel(131072);
        let tasks: Vec<_> = keys
            .iter()
            .map(|key| {
                let key = key.clone();
                let mut redis_conn = redis_conn.clone();
                let sender: Sender<T> = sender.clone();
                tokio::spawn(async move {
                    if let Ok(item_json) = cmd("GET")
                        .arg(&[key])
                        .query_async::<_, String>(&mut redis_conn)
                        .await
                    {
                        let item = serde_json::from_str::<T>(&item_json)
                            .expect("Unable to deserialize JSON to T");
                        sender.send(item).await.expect("unable to send");
                    }
                })
            })
            .collect();
        future::join_all(tasks).await;
        let mut result = Vec::default();
        while !receiver.is_empty() {
            result.push(receiver.recv().await.unwrap());
        }
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
