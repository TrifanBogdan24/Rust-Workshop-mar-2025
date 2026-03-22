use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    extract::State
};

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::AppState;

pub fn register() -> Router<Arc<RwLock<AppState>>> {
    Router::new().route("/", get(health_check))
}

#[utoipa::path(
    get,
    path = "/healtcheck",
    responses(
        (status = StatusCode::OK, body = String , description = "Helth check endpoint"),
    ),
    tag = "Healthcheck"
)]
async fn health_check(
    State(_state): State<Arc<RwLock<AppState>>>
) -> impl IntoResponse {
    // Task 1: implement function
    (StatusCode::OK, "Server is running")
}
