use crate::controller;
use axum::routing::{get, post};
use axum::Router;
use std::env;

pub async fn run() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // define application and routes
    let app = Router::new()
        .route("/", get(crate::controller::root_controller::root))
        .route(
            "/device",
            post(controller::device_controller::create_device),
        );

    // run app with hyper, listening globally on port 8081 or process.env.PORT
    let port = env::var("PORT")
        .unwrap_or("8081".parse().unwrap())
        .parse::<u16>()
        .expect("Invalid PORT number");
    let addr = format!("{}{}", "0.0.0.0", format!(":{}", port));
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
    log::info!("Server listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
