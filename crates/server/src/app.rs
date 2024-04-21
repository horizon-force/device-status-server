use crate::config::app_state::AppState;
use crate::controller;
use crate::service::redis_service::RedisService;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use async_std::sync::Weak;
use axum::routing::{get, put};
use axum::Router;
use deadpool_redis::redis::cmd;
use deadpool_redis::{Config, Pool, Runtime};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn run() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // initialize redis connection
    // TODO: rather than initializing redis here, there should be a generate "Redis Service" that abstracts everything
    let redis_service = RedisService::default();
    let redis_cfg = create_redis_config();
    let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

    // start cron scheduler to periodically store all devices in-memory
    let device_cache: Arc<RwLock<HashMap<String, String>>> = Arc::new(Default::default());
    start_scheduler(Arc::clone(&device_cache), redis_pool.clone())
        .await
        .expect("Unable to start cron scheduler");

    // global app state
    let app_state = AppState {
        redis_pool: redis_pool.clone(),
        device_cache: Arc::downgrade(&device_cache),
        redis_service,
    };

    // define application and routes
    let app = Router::new()
        .route("/", get(controller::root_controller::root))
        .route(
            "/api/v0/device",
            get(controller::device_controller::get_devices),
        )
        .route(
            "/api/v0/device/:id",
            get(controller::device_controller::get_device),
        )
        .route(
            "/api/v0/device",
            put(controller::device_controller::put_device),
        )
        .with_state(app_state);

    // run app with hyper, listening globally on port 8081 or process.env.PORT
    let port = env::var("PORT")
        .unwrap_or("8081".parse().unwrap())
        .parse::<u16>()
        .expect("Invalid PORT number");
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
    log::info!("Server listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

pub async fn start_scheduler(
    device_cache: Arc<RwLock<HashMap<String, String>>>,
    redis_pool: Pool,
) -> anyhow::Result<(), anyhow::Error> {
    // Update device cache once
    update_device_cache(&redis_pool, &Arc::downgrade(&device_cache)).await;

    // cron job to store all device data in-memory repeatedly over time
    let sched = JobScheduler::new().await?;
    sched
        .add(
            Job::new_repeated_async(Duration::from_secs(10), move |uuid, mut l| {
                let redis_pool = redis_pool.clone();
                let device_cache_weak = Arc::downgrade(&device_cache);

                Box::pin(async move {
                    // Update device cache
                    update_device_cache(&redis_pool, &device_cache_weak).await;

                    // Query the next execution time for this job
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => log::info!("Next time for job is {:?}", ts),
                        _ => log::info!("Could not get next tick for job"),
                    }
                })
            })
            .expect("Unable to create cron job"),
        )
        .await?;
    sched.start().await?;
    Ok(())
}

fn create_redis_config() -> Config {
    Config::from_url(env::var("REDIS__URL").unwrap_or("redis://localhost:10001".parse().unwrap()))
}

async fn update_device_cache(
    redis_pool: &Pool,
    device_cache_weak: &Weak<RwLock<HashMap<String, String>>>,
) {
    let mut redis_conn = redis_pool
        .get()
        .await
        .expect("Unable to acquire Redis connection from connection pool");
    match cmd("KEYS").arg("*").query_async(&mut redis_conn).await {
        Ok(res) => {
            let device_cache = device_cache_weak
                .upgrade()
                .expect("Device cache is no longer available");
            device_cache.write().await.clear();
            let keys: Vec<String> = res;
            for key in keys {
                match cmd("GET")
                    .arg(&[key.clone()])
                    .query_async(&mut redis_conn)
                    .await
                {
                    Ok(device_json) => {
                        let device_json: String = device_json;
                        device_cache.write().await.insert(key, device_json);
                    }
                    Err(err) => {
                        log::error!("redis error for getting vale {err}");
                    }
                }
            }
            log::info!(
                "Size of device cache is {}",
                device_cache.read().await.len()
            );
        }
        Err(err) => {
            log::error!("redis error for getting keys {err}");
        }
    }
}
