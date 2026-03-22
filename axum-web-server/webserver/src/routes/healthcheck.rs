use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::get
};

pub fn register() -> Router {
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
async fn health_check() -> impl IntoResponse {
    // Task 1: implement function
    (StatusCode::OK, "Server is running")
}
