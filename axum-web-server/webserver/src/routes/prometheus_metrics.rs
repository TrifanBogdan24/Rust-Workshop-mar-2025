use axum::{Router, routing::get, extract::State, response::IntoResponse};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::AppState;

pub fn register() -> Router<Arc<RwLock<AppState>>> {
    Router::new()
        .route("/", get(prometheus_metrics_handler))
}

#[utoipa::path(
    get,
    path = "/prometheus-metrics",
    responses(
        (
            status = StatusCode::OK,
            body = String,
            description = "All system metrics in OpenMetrics text format",
            content_type = "text/plain; version=0.0.4"
        ),
    ),
    tag = "System Metrics",
    summary = "Export metrics for Prometheus",
    description = "Provides real-time CPU, Memory, Disk, and Process stats from internal cache for Prometheus scraping."
)]
async fn prometheus_metrics_handler(
    State(state): State<Arc<RwLock<AppState>>>
) -> impl IntoResponse {
    let s = state.read().await;
    let data = &s.metrics;
    let mut msg = String::new();

    // SYSTEM & MEMORY
    msg.push_str(&format!("system_uptime_seconds {}\n", data.system.uptime));
    msg.push_str(&format!("system_memory_used_bytes {}\n", data.memory.used));
    msg.push_str(&format!("system_memory_total_bytes {}\n", data.memory.total));

    // CPU
    msg.push_str(&format!("system_cpu_usage_average {}\n", data.cpu.cpu_usage));
    for core in &data.cpu.cores {
        msg.push_str(&format!(
            "system_cpu_core_usage{{core=\"{}\", brand=\"{}\"}} {}\n",
            core.name, core.brand, core.usage
        ));
        msg.push_str(&format!(
            "system_cpu_core_frequency{{core=\"{}\"}} {}\n",
            core.name, core.frequency
        ));
    }

    // DISKS
    for disk in &data.disks {
        msg.push_str(&format!(
            "system_disk_available_bytes{{name=\"{}\", removable=\"{}\"}} {}\n",
            disk.name, disk.is_removable, disk.available_space
        ));
        msg.push_str(&format!("system_disk_total_bytes{{name=\"{}\"}} {}\n", disk.name, disk.total_space));
    }

    // Top 10 PROCESSES
    let mut procs = data.process.clone();
    procs.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    
    for p in procs.iter().take(10) {
        msg.push_str(&format!(
            "system_process_cpu_usage{{name=\"{}\", pid=\"{}\"}} {}\n",
            p.name, p.pid, p.cpu_usage
        ));
        msg.push_str(&format!(
            "system_process_memory_bytes{{name=\"{}\", pid=\"{}\"}} {}\n",
            p.name, p.pid, p.memory
        ));
    }

    msg
}
