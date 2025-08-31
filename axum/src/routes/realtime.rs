use std::time::Duration;
use axum::{
    Router,
    response::sse::{Sse, Event},
    routing::get,
};
use futures_core::stream::Stream;
use tokio_stream::{StreamExt, wrappers::IntervalStream};
use serde_json::json;

use crate::metrics;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json}
};

pub fn register() -> Router {
    Router::new()
        .route("/", get(realtime_metrics))
        .route("/{kind}", get(realtime_metric))

}

async fn realtime_metrics() -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    // Create a 1-second interval stream
    let interval = IntervalStream::new(tokio::time::interval(Duration::from_secs(1)));

    // Map each tick into an SSE event with system summary metrics
    let stream = interval.then(|_| async {
        let mut sys = metrics::init().await;
        let summary = metrics::Summary::generate(&mut sys);

        let data = json!(summary).to_string();
        Ok(Event::default().data(data))
    });

    Sse::new(stream)
}


async fn realtime_metric(Path(kind): Path<String>) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let interval = IntervalStream::new(tokio::time::interval(Duration::from_secs(1)));

    let stream = interval.then(move |_| {
        let kind = kind.clone();
        async move {
            let mut sys = metrics::init().await;

            let data = match kind.as_str() {
                "system" => json!(metrics::System::generate()),
                "process" => json!(metrics::Process::generate(&mut sys)),
                "memory" => json!(metrics::Memory::generate(&mut sys)),
                "cpu" => json!(metrics::Cpu::generate(&mut sys)),
                "disk" => json!(metrics::Disk::generate()),
                "summary" => json!(metrics::Summary::generate(&mut sys)),
                _ => json!({ "error": "Invalid metric type" }),
            };

            Ok(Event::default().data(data.to_string()))
        }
    });

    Sse::new(stream)
}
