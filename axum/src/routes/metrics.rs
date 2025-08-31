use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get
};
use crate::metrics;
use serde_json::json;

pub fn register() -> Router {
    Router::new()
        .route("/", get(get_metrics))
        .route("/{kind}", get(get_metric))
}

async fn get_metrics() -> impl IntoResponse {
    // Task 2: implement function
    let mut sys = metrics::init().await;
    let summary = metrics::Summary::generate(&mut sys);

    (StatusCode::OK, Json(summary))
}

async fn get_metric(Path(kind): Path<String>) -> impl IntoResponse {
    // Task 2: implement function
    let kind = kind.to_lowercase();
    let mut sys = metrics::init().await;

    let data = match kind.as_str() {
        "system" => json!(metrics::System::generate()),
        "process" => json!(metrics::Process::generate(&mut sys)),
        "memory" => json!(metrics::Memory::generate(&mut sys)),
        "cpu" => json!(metrics::Cpu::generate(&mut sys)),
        "disk" => json!(metrics::Disk::generate()),
        "summary" => json!(metrics::Summary::generate(&mut sys)),
        _ => return (StatusCode::NOT_FOUND, Json(json!({ "error": "Invalid metric type" }))),
    };

    (StatusCode::OK, Json(data))
}
