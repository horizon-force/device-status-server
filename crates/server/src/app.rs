use crate::config::app_state::AppState;
use crate::controller;
use crate::model::device::Device;
use crate::service::device_cache_service::DeviceCacheService;
use crate::service::redis_service::RedisService;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use axum::routing::{get, put};
use axum::Router;
use std::collections::HashMap;
use std::env;

pub async fn run() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // in-memory cache of devices
    let device_cache: Arc<RwLock<HashMap<String, Device>>> = Arc::new(Default::default());

    // initialize redis connection
    let redis_service = RedisService::default();

    // global app state
    let app_state = AppState {
        device_cache: Arc::downgrade(&device_cache),
        redis_service,
    };

    // start cron scheduler to periodically store all devices in-memory
    DeviceCacheService::run(app_state.clone())
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
