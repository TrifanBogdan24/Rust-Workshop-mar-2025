use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System as SystemInfo};
use utoipa::{ToSchema};
use serde_json::{json, Value};

pub async fn init() -> sysinfo::System {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    sys.refresh_cpu_all();

    sys
}

#[derive(Deserialize, ToSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    System,
    Process,
    Memory,
    Cpu,
    Disk,
    Summary
}


impl Kind {
    pub async fn generate_json_response(&self) -> Value {
        let mut sys = init().await;

        match self {
            Self::System => json!(System::generate()),
            Self::Process => json!(Process::generate(&mut sys)),
            Self::Memory => json!(Memory::generate(&mut sys)),
            Self::Cpu => json!(Cpu::generate(&mut sys)),
            Self::Disk => json!(Disk::generate()),
            Self::Summary => json!(Summary::generate(&mut sys)),
        }
    }
}

// Task 2: add derive
#[derive(Serialize, Deserialize, ToSchema)]
pub struct System {
    name: String,
    kernel_version: String,
    os_version: String,
    host_name: String,
    uptime: u64,
}

impl System {
    pub fn generate() -> Self {
        // Task 2: implement function
        Self {
            name: SystemInfo::name().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: SystemInfo::kernel_version()
                .unwrap_or_else(|| "Unknown".to_string()),
            os_version: SystemInfo::os_version().unwrap_or_else(|| "Unknown".to_string()),
            host_name: SystemInfo::host_name().unwrap_or_else(|| "Unknown".to_string()),
            uptime: SystemInfo::uptime(),
        }
    }
}

// Task 2: add derive
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Process {
    pid: u32,
    name: String,
    memory: u64,
    cpu_usage: f32,
    run_time: u64,
}

impl Process {
    pub fn generate(sys: &SystemInfo) -> Vec<Self> {
        // Task 2: implement function
        sys.processes()
            .values()
            .map(|p| Process {
                pid: p.pid().as_u32(),
                name: p.name().to_string_lossy().to_string(),
                memory: p.memory(),
                cpu_usage: p.cpu_usage(),
                run_time: p.run_time(),
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Memory {
    used: u64,
    total: u64,
}

impl Memory {
    pub fn generate(sys: &SystemInfo) -> Self {
        // Task 2: implement function
        Self {
            used: sys.used_memory(),
            total: sys.total_memory(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CoreMetrics {
    name: String,
    brand: String,
    usage: f32,
    frequency: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Cpu {
    cpu_usage: f32,
    cores: Vec<CoreMetrics>,
}

impl Cpu {
    pub fn generate(sys: &SystemInfo) -> Self {
        // Task 2: implement function
        let cpus = sys.cpus();
        let cores: Vec<CoreMetrics> = cpus
            .iter()
            .enumerate()
            .map(|(i, cpu)| CoreMetrics {
                name: format!("Core {}", i),
                brand: cpu.brand().to_string(),
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
            })
            .collect();

        let cpu_usage = if !cpus.is_empty() {
            cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
        } else {
            0.0
        };

        Self { cpu_usage, cores }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Disk {
    name: String,
    available_space: u64,
    total_space: u64,
    is_removable: bool,
}

impl Disk {
    pub fn generate() -> Vec<Self> {
        // Task 2: implement function
        // Use sysinfo's Disks
        let disks = Disks::new_with_refreshed_list();

        disks
            .list()
            .iter()
            .map(|disk| Disk {
                name: disk.name().to_string_lossy().to_string(),
                available_space: disk.available_space(),
                total_space: disk.total_space(),
                is_removable: disk.is_removable(),
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Summary {
    system: System,
    process: Vec<Process>,
    memory: Memory,
    cpu: Cpu,
    disk: Vec<Disk>,
}

impl Summary {
    pub fn generate(sys: &SystemInfo) -> Self {
        // Task 2: implement function
        Self {
            system: System::generate(),
            process: Process::generate(sys),
            memory: Memory::generate(sys),
            cpu: Cpu::generate(sys),
            disk: Disk::generate(),
        }
    }
}
