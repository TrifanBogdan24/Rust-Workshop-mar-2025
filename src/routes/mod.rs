use axum::{Router, routing::get};

mod healthcheck;
mod metrics;

pub fn app() -> Router {
    Router::new()
        .route("/", get(async || "Hello, World!"))
        .nest("/healthcheck", healthcheck::register())        // Task 1
        .nest("/metrics", metrics::register())                       // Task 2
}
