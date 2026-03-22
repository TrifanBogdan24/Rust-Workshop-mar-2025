use axum::{Router, routing::get};

mod healthcheck;
mod metrics;
mod realtime;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::routes::healthcheck::health_check,
    crate::routes::metrics::get_metrics,
    crate::routes::metrics::get_metric,
))]
struct ApiDoc;

pub fn app() -> Router {
    Router::new()
        .route("/", get(async || "Hello, World!"))
        .nest("/healthcheck", healthcheck::register()) // Task 1
        .nest("/metrics", metrics::register()) // Task 2
        .nest("/realtime", realtime::register())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
}
