pub struct SystemMonitor {
    cpu: CpuInfo,
    memory: MemoryInfo,
    gpu: GpuInfo,
    disk: DiskInfo,
    network: NetworkInfo,
    ai: AIActivity,
}

pub struct CpuInfo {
    pub usage_percent: f32,
    pub temperature: f32,
    pub frequency: f32,
    pub cores: u32,
    pub load: [f32; 3],
}

pub struct MemoryInfo {
    pub total_gb: f32,
    pub used_gb: f32,
    pub available_gb: f32,
    pub swap_used: f32,
    pub swap_total: f32,
}

pub struct GpuInfo {
    pub name: String,
    pub usage_percent: f32,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
    pub temperature: f32,
}

pub struct DiskInfo {
    pub total_gb: f64,
    pub used_gb: f64,
    pub available_gb: f64,
    pub read_speed: f64,
    pub write_speed: f64,
}

pub struct NetworkInfo {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_speed: f64,
    pub tx_speed: f64,
    pub connections: u32,
}

pub struct AIActivity {
    pub active_agents: u32,
    pub memory_nodes: u64,
    pub memory_edges: u64,
    pub tasks_queued: u32,
    pub tasks_completed: u64,
    pub learning_active: bool,
    pub automation_count: u32,
}

pub struct Temperature {
    pub name: String,
    pub temp_c: f64,
    pub max_temp_c: f64,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            cpu: CpuInfo {
                usage_percent: 0.0,
                temperature: 0.0,
                frequency: 0.0,
                cores: num_cpus(),
                load: [0.0, 0.0, 0.0],
            },
            memory: MemoryInfo {
                total_gb: 0.0,
                used_gb: 0.0,
                available_gb: 0.0,
                swap_used: 0.0,
                swap_total: 0.0,
            },
            gpu: GpuInfo {
                name: String::new(),
                usage_percent: 0.0,
                vram_used_mb: 0,
                vram_total_mb: 0,
                temperature: 0.0,
            },
            disk: DiskInfo {
                total_gb: 0.0,
                used_gb: 0.0,
                available_gb: 0.0,
                read_speed: 0.0,
                write_speed: 0.0,
            },
            network: NetworkInfo {
                rx_bytes: 0,
                tx_bytes: 0,
                rx_speed: 0.0,
                tx_speed: 0.0,
                connections: 0,
            },
            ai: AIActivity {
                active_agents: 0,
                memory_nodes: 0,
                memory_edges: 0,
                tasks_queued: 0,
                tasks_completed: 0,
                learning_active: true,
                automation_count: 0,
            },
        }
    }

    pub fn display(&self) {
        // Display real-time system overview
        // CPU, RAM, GPU, Disk, Network, AI Activity
    }

    pub fn refresh_all(&mut self) {
        self.read_cpu();
        self.read_memory();
        self.read_gpu();
        self.read_disk();
        self.read_network();
    }

    pub fn cpu(&self) -> &CpuInfo { &self.cpu }
    pub fn memory(&self) -> &MemoryInfo { &self.memory }
    pub fn gpu(&self) -> &GpuInfo { &self.gpu }
    pub fn disk(&self) -> &DiskInfo { &self.disk }
    pub fn network(&self) -> &NetworkInfo { &self.network }
    pub fn ai(&self) -> &AIActivity { &self.ai }
    pub fn temperatures(&self) -> Vec<Temperature> { Vec::new() }

    fn read_cpu(&mut self) {
        // Read /proc/stat for CPU usage
        // Read /sys/class/thermal for temperature
    }

    fn read_memory(&mut self) {
        // Read /proc/meminfo
    }

    fn read_gpu(&mut self) {
        // Query GPU via DRM or NVML
    }

    fn read_disk(&mut self) {
        // Read /proc/diskstats
    }

    fn read_network(&mut self) {
        // Read /proc/net/dev
    }
}

fn num_cpus() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(4)
}

pub fn format_bytes(bytes: f64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes;
    let mut unit_idx = 0;
    while size > 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.1} {}", size, UNITS[unit_idx])
}
