use crate::controller;
use crate::controller::device_controller::PutDeviceRequest;
use axum::routing::{get, put};
use axum::{Json, Router};
use deadpool_redis::{Config, Runtime};
use std::env;

pub async fn run() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // initialize redis connection
    let redis_cfg = Config::from_url(
        env::var("REDIS__URL").unwrap_or("redis://localhost:10001".parse().unwrap()),
    );
    let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

    // define application and routes
    let app = Router::new()
        .route("/", get(crate::controller::root_controller::root))
        .route(
            "/api/v0/device",
            put(move |body: Json<PutDeviceRequest>| {
                controller::device_controller::put_device(body, redis_pool)
            }),
        );

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
