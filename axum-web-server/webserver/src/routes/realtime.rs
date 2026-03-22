use axum::{
    Router,
    response::sse::{Event, Sse},
    routing::get,
};
use futures_core::stream::Stream;
use serde_json::{Value, json};
use std::time::Duration;
use tokio_stream::{StreamExt, wrappers::IntervalStream};

use crate::metrics;
use axum::{
    extract::Path,
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

async fn realtime_metric(
    Path(kind): Path<metrics::Kind>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let interval = IntervalStream::new(tokio::time::interval(Duration::from_secs(1)));

    let stream = interval.then(move |_| {
        let cloned_kind = kind.clone();
        async move {
            let response: Value = cloned_kind.generate_json_response().await;
            Ok(Event::default().data(response.to_string()))
        }
    });

    Sse::new(stream)
}
