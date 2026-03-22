use crate::metrics;
use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};
use serde_json::Value;


pub fn register() -> Router {
    Router::new()
        .route("/", get(get_metrics))
        .route("/{kind}", get(get_metric))
}

#[utoipa::path(
    get,
    path = "/metrics",
    responses(
        (status = StatusCode::OK, body = metrics::Summary, description = "Fetch Summary System Metrics"),
    ),
    tag = "System Metrics"
)]
async fn get_metrics() -> impl IntoResponse {
    // Task 2: implement function
    let mut sys = metrics::init().await;
    let summary = metrics::Summary::generate(&mut sys);

    (StatusCode::OK, Json(summary))
}


#[utoipa::path(
    get,
    path = "/metrics/{kind}",
    params(
        ("kind" = metrics::Kind, Path, description = "Type of metric to retrieve")
    ),
    responses(
        (status = StatusCode::OK, body = metrics::System, description = "Fetch all System Metrics"),
        (status = StatusCode::OK, body = metrics::Process, description = "Fetch System Processes Metrics"),
        (status = StatusCode::OK, body = metrics::Memory, description = "Fetch System Memory Metrics"),
        (status = StatusCode::OK, body = metrics::Cpu, description = "Fetch System CPU Metrics"),
        (status = StatusCode::OK, body = metrics::Disk, description = "Fetch System Disk Metrics"),
        (status = StatusCode::OK, body = metrics::Summary, description = "Fetch Summary System Metrics"),
    ),
    tag = "System Metrics"
)]
async fn get_metric(Path(kind): Path<metrics::Kind>) -> impl IntoResponse {
    // Task 2: implement function
    let response: Value = kind.generate_json_response().await;
    (StatusCode::OK, Json(response))
}
