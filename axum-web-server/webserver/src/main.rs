use axum;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::metrics::{Summary};

mod metrics;
mod routes;


struct AppState {
    metrics: crate::metrics::Summary,
}

async fn start_background_monitoring(state: Arc<RwLock<AppState>>) {
    let mut sys = sysinfo::System::new_all();
    
    loop {
        sys.refresh_all();
        
        let new_summary = crate::metrics::Summary::generate(&sys);
        
        {
            let mut lock = state.write().await;
            lock.metrics = new_summary;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
    }
}


#[tokio::main]
async fn main() {
    let initial_sys = sysinfo::System::new_all();
    let state = Arc::new(RwLock::new(AppState {
        metrics: Summary::generate(&initial_sys),
    }));

    let monitor_state = Arc::clone(&state);
    tokio::spawn(async move {
        start_background_monitoring(monitor_state).await;
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!(
        "Server running on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, routes::webserver(Arc::clone(&state))).await.unwrap();
}
