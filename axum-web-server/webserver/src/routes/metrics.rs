use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};
use serde_json::Value;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::metrics;
use crate::AppState;

pub fn register() -> Router<Arc<RwLock<AppState>>> {
    Router::new()
        .route("/", get(get_metrics))
        .route("/{kind}", get(get_metric))
}

#[utoipa::path(
    get,
    path = "/metrics",
    responses(
        (status = StatusCode::OK, body = metrics::Summary, description = "Fetch All System Metrics (Summary)"),
    ),
    tag = "System Metrics"
)]
async fn get_metrics(
    State(state): State<Arc<RwLock<AppState>>>
) -> impl IntoResponse {
    // Task 2: implement function
    let lock = state.read().await;
    let summary = lock.metrics.clone(); 

    (StatusCode::OK, Json(summary))
}


#[utoipa::path(
    get,
    path = "/metrics/{kind}",
    params(
        ("kind" = metrics::Kind, Path, description = "Type of metric to retrieve")
    ),
    responses(
        (status = StatusCode::OK, body = metrics::System, description = "Fetch System Info"),
        (status = StatusCode::OK, body = metrics::Process, description = "Fetch System Processes Metrics"),
        (status = StatusCode::OK, body = metrics::Memory, description = "Fetch System Memory Metrics"),
        (status = StatusCode::OK, body = metrics::Cpu, description = "Fetch System CPU Metrics"),
        (status = StatusCode::OK, body = metrics::Disk, description = "Fetch System Disk Metrics"),
        (status = StatusCode::OK, body = metrics::Summary, description = "Fetch All System Metrics (Summary)"),
    ),
    tag = "System Metrics"
)]
async fn get_metric(
    State(_state): State<Arc<RwLock<AppState>>>,
    Path(kind): Path<metrics::Kind>
) -> impl IntoResponse {
    // Task 2: implement function
    let response: Value = kind.generate_json_response().await;
    (StatusCode::OK, Json(response))
}
