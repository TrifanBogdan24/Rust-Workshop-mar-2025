use axum::{
    Router,
    routing::get
};

use std::sync::Arc;
use tokio::sync::RwLock;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

mod healthcheck;
mod metrics;
mod realtime;
mod prometheus_metrics;

#[derive(OpenApi)]
#[openapi(paths(
    healthcheck::health_check,
    metrics::get_metrics,
    metrics::get_metric,
    prometheus_metrics::prometheus_metrics_handler,
))]
struct ApiDoc;



pub fn webserver(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/", get(async || "Hello, World!"))
        .nest("/healthcheck", healthcheck::register()) // Task 1
        .nest("/metrics", metrics::register()) // Task 2
        .nest("/realtime", realtime::register())
        .nest("/prometheus-metrics", prometheus_metrics::register())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}

