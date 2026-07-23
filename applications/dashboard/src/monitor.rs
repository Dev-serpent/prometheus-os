use std::fs;

pub struct SystemMonitor {
    cpu: CpuStats,
    memory: MemoryStats,
    gpu: GpuStats,
    disk: DiskStats,
    network: NetworkStats,
    processes: Vec<ProcessInfo>,
    temps: Vec<Temperature>,
    prev_cpu: Option<CpuSample>,
    prev_net: Option<NetSample>,
}

#[derive(Default, Clone)]
pub struct CpuStats {
    pub usage_percent: f64,
    pub temperature_c: f64,
    pub frequency_mhz: f64,
    pub core_count: u32,
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,
}

#[derive(Default, Clone)]
pub struct MemoryStats {
    pub total_kb: u64,
    pub used_kb: u64,
    pub free_kb: u64,
    pub available_kb: u64,
    pub cached_kb: u64,
    pub buffers_kb: u64,
    pub swap_total_kb: u64,
    pub swap_used_kb: u64,
    pub swap_free_kb: u64,
    pub percent_used: f64,
}

#[derive(Default, Clone)]
pub struct GpuStats {
    pub name: String,
    pub usage_percent: f64,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
    pub temperature_c: f64,
    pub power_watts: f64,
}

#[derive(Default, Clone)]
pub struct DiskStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub read_bytes_per_sec: f64,
    pub write_bytes_per_sec: f64,
}

#[derive(Default, Clone)]
pub struct NetworkStats {
    pub rx_speed: f64,
    pub tx_speed: f64,
    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
    pub connections: u32,
}

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f64,
    pub memory_kb: u64,
    pub state: String,
    pub user: String,
    pub threads: u32,
}

#[derive(Clone)]
pub struct Temperature {
    pub name: String,
    pub temp_c: f64,
    pub max_temp_c: f64,
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

struct CpuSample {
    total: u64,
    work: u64,
    idle: u64,
    iowait: u64,
}

struct NetSample {
    rx: u64,
    tx: u64,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let count = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(4);
        Self {
            cpu: CpuStats { core_count: count, ..Default::default() },
            memory: MemoryStats::default(),
            gpu: GpuStats::default(),
            disk: DiskStats::default(),
            network: NetworkStats::default(),
            processes: Vec::new(),
            temps: Vec::new(),
            prev_cpu: None,
            prev_net: None,
        }
    }

    pub fn refresh_all(&mut self) {
        self.read_cpu();
        self.read_memory();
        self.read_gpu();
        self.read_disk();
        self.read_network();
        self.read_processes();
        self.read_temperatures();
    }

    fn read_cpu(&mut self) {
        if let Ok(content) = fs::read_to_string("/proc/stat") {
            for line in content.lines() {
                if line.starts_with("cpu") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() < 5 { continue; }
                    let user: u64 = parts[1].parse().unwrap_or(0);
                    let nice: u64 = parts[2].parse().unwrap_or(0);
                    let system: u64 = parts[3].parse().unwrap_or(0);
                    let idle: u64 = parts[4].parse().unwrap_or(0);
                    let iowait: u64 = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);
                    let total = user + nice + system + idle + iowait;
                    let work = user + nice + system;
                    let sample = CpuSample { total, work, idle, iowait };
                    if parts[0] == "cpu" {
                        if let Some(prev) = &self.prev_cpu {
                            let dt = total.saturating_sub(prev.total);
                            let dw = work.saturating_sub(prev.work);
                            if dt > 0 {
                                self.cpu.usage_percent = (dw as f64 / dt as f64) * 100.0;
                            }
                        }
                        self.prev_cpu = Some(sample);
                    }
                }
            }
        }
        if let Ok(content) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 3 {
                self.cpu.load_1 = parts[0].parse().unwrap_or(0.0);
                self.cpu.load_5 = parts[1].parse().unwrap_or(0.0);
                self.cpu.load_15 = parts[2].parse().unwrap_or(0.0);
            }
        }
        if let Ok(content) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq") {
            self.cpu.frequency_mhz = content.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
        }
        for hwmon in 0..8 {
            let path = format!("/sys/class/hwmon/hwmon{}/name", hwmon);
            if let Ok(name) = fs::read_to_string(&path) {
                if name.trim() == "coretemp" || name.trim().contains("cpu") {
                    if let Ok(temp) = fs::read_to_string(format!("/sys/class/hwmon/hwmon{}/temp1_input", hwmon)) {
                        self.cpu.temperature_c = temp.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                        break;
                    }
                }
            }
        }
    }

    fn read_memory(&mut self) {
        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 { continue; }
                let key = parts[0].trim_end_matches(':');
                let val = parts[1].parse::<u64>().unwrap_or(0);
                match key {
                    "MemTotal" => self.memory.total_kb = val,
                    "MemFree" => self.memory.free_kb = val,
                    "MemAvailable" => self.memory.available_kb = val,
                    "Buffers" => self.memory.buffers_kb = val,
                    "Cached" => self.memory.cached_kb = val,
                    "SwapTotal" => self.memory.swap_total_kb = val,
                    "SwapFree" => self.memory.swap_free_kb = val,
                    _ => {}
                }
            }
            self.memory.used_kb = self.memory.total_kb
                .saturating_sub(self.memory.available_kb)
                .saturating_sub(self.memory.buffers_kb)
                .saturating_sub(self.memory.cached_kb);
            self.memory.swap_used_kb = self.memory.swap_total_kb
                .saturating_sub(self.memory.swap_free_kb);
            if self.memory.total_kb > 0 {
                self.memory.percent_used = (self.memory.total_kb - self.memory.available_kb) as f64
                    / self.memory.total_kb as f64 * 100.0;
            }
        }
    }

    fn read_gpu(&mut self) {
        for hwmon in 0..8 {
            let base = format!("/sys/class/hwmon/hwmon{}", hwmon);
            let name_path = format!("{}/name", base);
            if let Ok(name) = fs::read_to_string(&name_path) {
                let n = name.trim();
                if n == "amdgpu" {
                    if let Ok(temp) = fs::read_to_string(format!("{}/temp1_input", base)) {
                        self.gpu.temperature_c = temp.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                    }
                    if let Ok(power) = fs::read_to_string(format!("{}/power1_average", base)) {
                        self.gpu.power_watts = power.trim().parse::<f64>().unwrap_or(0.0) / 1000000.0;
                    }
                    self.gpu.name = "AMD GPU".to_string();
                    break;
                }
                if n == "i915" {
                    if let Ok(temp) = fs::read_to_string(format!("{}/temp1_input", base)) {
                        self.gpu.temperature_c = temp.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                    }
                    self.gpu.name = "Intel GPU".to_string();
                    break;
                }
            }
        }
        if let Ok(output) = std::process::Command::new("nvidia-smi")
            .args(["--query-gpu=name,utilization.gpu,memory.used,memory.total,temperature.gpu",
                   "--format=csv,noheader,nounits"])
            .output()
        {
            if output.status.success() {
                let out = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = out.trim().split(", ").collect();
                if parts.len() >= 5 {
                    self.gpu.name = parts[0].to_string();
                    self.gpu.usage_percent = parts[1].parse().unwrap_or(0.0);
                    self.gpu.vram_used_mb = parts[2].parse().unwrap_or(0);
                    self.gpu.vram_total_mb = parts[3].parse().unwrap_or(0);
                    self.gpu.temperature_c = parts[4].parse().unwrap_or(0.0);
                }
            }
        }
    }

    fn read_disk(&mut self) {
        if let Ok(content) = fs::read_to_string("/proc/mounts") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] != "rootfs" && parts[2] != "proc" && parts[2] != "sysfs" {
                    let mount = parts[1];
                    if mount.starts_with('/') && !mount.starts_with("/sys") && !mount.starts_with("/proc")
                        && !mount.starts_with("/dev") && !mount.starts_with("/run")
                    {
                        if let Some(usage) = fs_usage(mount) {
                            self.disk.total_bytes += usage.total;
                            self.disk.used_bytes += usage.used;
                            self.disk.free_bytes += usage.free;
                        }
                    }
                }
            }
        }
    }

    fn read_network(&mut self) {
        if let Ok(content) = fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    let name = parts[0].trim_end_matches(':');
                    if name == "lo" { continue; }
                    let rx: u64 = parts[1].parse().unwrap_or(0);
                    let tx: u64 = parts[9].parse().unwrap_or(0);
                    self.network.total_rx_bytes += rx;
                    self.network.total_tx_bytes += tx;
                }
            }
        }
        self.network.connections = 0;
    }

    fn read_processes(&mut self) {
        self.processes.clear();
        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                let pid_str = entry.file_name();
                let pid: u32 = match pid_str.to_str().and_then(|s| s.parse().ok()) {
                    Some(p) => p,
                    None => continue,
                };
                let stat_path = entry.path().join("stat");
                let status_path = entry.path().join("status");
                if let Ok(content) = fs::read_to_string(&stat_path) {
                    let parts: Vec<&str> = content.split_whitespace().collect();
                    if parts.len() >= 13 {
                        let name = parts[1].trim_matches('(').trim_matches(')').to_string();
                        let state = parts[2].to_string();
                        let threads: u32 = parts.get(19).and_then(|s| s.parse().ok()).unwrap_or(0);
                        let mem_kb = if let Ok(status) = fs::read_to_string(&status_path) {
                            let mut found = 0;
                            for line in status.lines() {
                                if line.starts_with("VmRSS:") {
                                    found = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                                    break;
                                }
                            }
                            found
                        } else { 0 };
                        self.processes.push(ProcessInfo {
                            pid,
                            name,
                            cpu_usage: 0.0,
                            memory_kb: mem_kb,
                            state,
                            user: String::new(),
                            threads,
                        });
                    }
                }
            }
        }
        self.processes.sort_by(|a, b| b.memory_kb.cmp(&a.memory_kb));
        self.processes.truncate(100);
    }

    fn read_temperatures(&mut self) {
        self.temps.clear();
        for hwmon in 0..8 {
            let base = format!("/sys/class/hwmon/hwmon{}", hwmon);
            let name_path = format!("{}/name", base);
            if let Ok(name) = fs::read_to_string(&name_path) {
                for i in 1..8 {
                    let input_path = format!("{}/temp{}_input", base, i);
                    if let Ok(temp) = fs::read_to_string(&input_path) {
                        let c = temp.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                        if c > 0.0 {
                            self.temps.push(Temperature {
                                name: format!("{} {}", name.trim(), i),
                                temp_c: c,
                                max_temp_c: 100.0,
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn cpu(&self) -> &CpuStats { &self.cpu }
    pub fn memory(&self) -> &MemoryStats { &self.memory }
    pub fn gpu(&self) -> &GpuStats { &self.gpu }
    pub fn disk(&self) -> &DiskStats { &self.disk }
    pub fn network(&self) -> &NetworkStats { &self.network }
    pub fn processes(&self) -> &[ProcessInfo] { &self.processes }
    pub fn temperatures(&self) -> &[Temperature] { &self.temps }
    pub fn ai(&self) -> &AIActivity {
        static AI: AIActivity = AIActivity {
            active_agents: 2,
            memory_nodes: 15234,
            memory_edges: 89112,
            tasks_queued: 3,
            tasks_completed: 452,
            learning_active: true,
            automation_count: 17,
        };
        &AI
    }
}

struct DiskUsage { total: u64, used: u64, free: u64 }

fn fs_usage(path: &str) -> Option<DiskUsage> {
    use std::ffi::CString;
    use std::mem;
    let cpath = CString::new(path).ok()?;
    unsafe {
        let mut stat: libc::statvfs = mem::zeroed();
        if libc::statvfs(cpath.as_ptr(), &mut stat) == 0 {
            let bs = stat.f_frsize as u64;
            let total = stat.f_blocks * bs;
            let free = stat.f_bfree * bs;
            return Some(DiskUsage { total, used: total - free, free });
        }
    }
    None
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
