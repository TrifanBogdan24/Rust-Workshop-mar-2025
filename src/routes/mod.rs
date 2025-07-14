use axum::{Router, routing::get};

mod healthcheck;

pub fn app() -> Router {
    Router::new()
        .route("/", get(async || "Hello, World!"))
        .nest("/healthcheck", healthcheck::register())
}
