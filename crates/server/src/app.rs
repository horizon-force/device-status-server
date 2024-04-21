use crate::config::app_state::AppState;
use crate::controller;
use crate::model::device::Device;
use crate::service::device_cache_service::DeviceCacheService;
use crate::service::redis_service::RedisService;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use axum::routing::{get, post};
use axum::Router;
use http::{HeaderValue, Method};
use std::collections::HashMap;
use std::env;
use tower_http::cors::{Any, CorsLayer};

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
        .expect("Unable to initialize device cache management service");

    // define CORS policy
    let consumer_endpoint =
        env::var("CORS_ALLOW_ORIGIN").unwrap_or("http://localhost:5173".parse().unwrap());
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::CONNECT,
            Method::TRACE,
            Method::OPTIONS,
        ])
        .allow_origin(consumer_endpoint.parse::<HeaderValue>().unwrap());

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
            post(controller::device_controller::post_device),
        )
        .layer(cors)
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
