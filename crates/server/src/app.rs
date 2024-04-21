use crate::config::app_state::AppState;
use crate::controller;
use crate::model::device::Device;
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

    // in-memory cache of devices
    let device_cache: Arc<RwLock<HashMap<String, String>>> = Arc::new(Default::default());

    // initialize redis connection
    let redis_service = RedisService::default();

    // global app state
    let app_state = AppState {
        device_cache: Arc::downgrade(&device_cache),
        redis_service,
    };

    // start cron scheduler to periodically store all devices in-memory
    start_scheduler(app_state.clone())
        .await
        .expect("Unable to start cron scheduler");

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

pub async fn start_scheduler(app_state: AppState) -> anyhow::Result<(), anyhow::Error> {
    // Update device cache once
    update_device_cache(app_state.clone()).await;

    // cron job to store all device data in-memory repeatedly over time
    let sched = JobScheduler::new().await?;
    sched
        .add(
            Job::new_repeated_async(Duration::from_secs(10), move |uuid, mut l| {
                let app_state = app_state.clone();

                Box::pin(async move {
                    // Update device cache
                    update_device_cache(app_state).await;

                    // Query the next execution time for this job
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => log::info!("Next time for cache refresh is {:?}", ts),
                        _ => log::info!("Could not get next tick for cache refresh job"),
                    }
                })
            })
            .expect("Unable to create cron job"),
        )
        .await?;
    sched.start().await?;
    Ok(())
}

async fn update_device_cache(app_state: AppState) {
    let devices: Vec<Device> = app_state
        .redis_service
        .get_all()
        .await
        .expect("Unable to get all devices");
    let device_cache = app_state
        .device_cache
        .upgrade()
        .expect("Device cache is no longer available");
    let mut write_guard = device_cache.write().await;
    for device in devices {
        // TODO: insert actual device into map
        write_guard.insert(device.id.clone(), device.id);
    }
}
