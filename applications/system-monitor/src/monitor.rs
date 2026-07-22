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
    pub user_percent: f64,
    pub system_percent: f64,
    pub iowait_percent: f64,
    pub temperature_c: f64,
    pub frequency_mhz: f64,
    pub core_count: u32,
    pub per_core: Vec<f64>,
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
    pub encoder_usage: f64,
    pub decoder_usage: f64,
    pub memory_temp_c: f64,
    pub power_watts: f64,
    pub clock_mhz: u64,
    pub memory_clock_mhz: u64,
}

#[derive(Default, Clone)]
pub struct DiskStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub read_bytes_per_sec: f64,
    pub write_bytes_per_sec: f64,
    pub read_ops_per_sec: f64,
    pub write_ops_per_sec: f64,
    pub partitions: Vec<PartitionInfo>,
}

#[derive(Clone)]
pub struct PartitionInfo {
    pub device: String,
    pub mount: String,
    pub fstype: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub used_percent: f64,
}

#[derive(Default, Clone)]
pub struct NetworkStats {
    pub rx_bytes_per_sec: f64,
    pub tx_bytes_per_sec: f64,
    pub rx_packets_per_sec: f64,
    pub tx_packets_per_sec: f64,
    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
    pub connections: u32,
    pub interfaces: Vec<InterfaceInfo>,
}

#[derive(Clone)]
pub struct InterfaceInfo {
    pub name: String,
    pub ip: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_speed: f64,
    pub tx_speed: f64,
    pub state: String,
}

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub state: String,
    pub user: String,
    pub threads: u32,
    pub fd_count: u32,
    pub start_time: u64,
}

#[derive(Clone)]
pub struct Temperature {
    pub name: String,
    pub temp_c: f64,
    pub max_temp_c: f64,
    pub critical_temp_c: f64,
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
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(4);

        Self {
            cpu: CpuStats {
                core_count: cpu_count,
                ..Default::default()
            },
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
        // Read /proc/stat for CPU usage
        if let Ok(content) = fs::read_to_string("/proc/stat") {
            let mut core_idx = 0;
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
                        // Overall CPU
                        if let Some(prev) = &self.prev_cpu {
                            let delta_total = total.saturating_sub(prev.total);
                            let delta_work = work.saturating_sub(prev.work);
                            let delta_idle = idle.saturating_sub(prev.idle);
                            let delta_iowait = iowait.saturating_sub(prev.iowait);

                            if delta_total > 0 {
                                self.cpu.usage_percent = (delta_work as f64 / delta_total as f64) * 100.0;
                                self.cpu.iowait_percent = (delta_iowait as f64 / delta_total as f64) * 100.0;
                            }
                        }
                        self.prev_cpu = Some(sample);
                    } else {
                        // Per-core
                        if core_idx < self.cpu.per_core.len() || core_idx < 128 {
                            // Simplified per-core tracking
                        }
                        core_idx += 1;
                    }
                }
            }
        }

        // Read load average
        if let Ok(content) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 3 {
                self.cpu.load_1 = parts[0].parse().unwrap_or(0.0);
                self.cpu.load_5 = parts[1].parse().unwrap_or(0.0);
                self.cpu.load_15 = parts[2].parse().unwrap_or(0.0);
            }
        }

        // Read CPU frequency
        if let Ok(content) = fs::read_to_string(
            "/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq"
        ) {
            self.cpu.frequency_mhz = content.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
        }

        // Read CPU temperature
        for hwmon in 0..8 {
            let path = format!("/sys/class/hwmon/hwmon{}/name", hwmon);
            if let Ok(name) = fs::read_to_string(&path) {
                if name.trim().starts_with("coretemp") || name.trim().contains("cpu") {
                    let temp_path = format!("/sys/class/hwmon/hwmon{}/temp1_input", hwmon);
                    if let Ok(temp) = fs::read_to_string(&temp_path) {
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
        // Try AMD GPU via hwmon
        for hwmon in 0..8 {
            let path = format!("/sys/class/hwmon/hwmon{}/name", hwmon);
            if let Ok(name) = fs::read_to_string(&path) {
                let name = name.trim();
                if name == "amdgpu" {
                    // Read GPU temperature
                    if let Ok(temp) = fs::read_to_string(
                        format!("/sys/class/hwmon/hwmon{}/temp1_input", hwmon)
                    ) {
                        self.gpu.temperature_c = temp.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                    }

                    // Read power
                    if let Ok(power) = fs::read_to_string(
                        format!("/sys/class/hwmon/hwmon{}/power1_average", hwmon)
                    ) {
                        self.gpu.power_watts = power.trim().parse::<f64>().unwrap_or(0.0) / 1000000.0;
                    }

                    self.gpu.name = "AMD GPU".to_string();
                    break;
                }
                if name == "i915" {
                    if let Ok(temp) = fs::read_to_string(
                        format!("/sys/class/hwmon/hwmon{}/temp1_input", hwmon)
                    ) {
                        self.gpu.temperature_c = temp.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                    }
                    self.gpu.name = "Intel GPU".to_string();
                    break;
                }
            }
        }

        // Try NVIDIA via nvidia-smi
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
        // Read filesystem disk usage
        if let Ok(content) = fs::read_to_string("/proc/mounts") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] != "rootfs" && parts[2] != "proc" && parts[2] != "sysfs" {
                    let mount = parts[1];
                    let fstype = parts[2];
                    if mount.starts_with('/') && !mount.starts_with("/sys") && !mount.starts_with("/proc")
                        && !mount.starts_with("/dev") && !mount.starts_with("/run")
                    {
                        if let Ok(usage) = fs_usage(mount) {
                            self.disk.partitions.push(PartitionInfo {
                                device: parts[0].to_string(),
                                mount: mount.to_string(),
                                fstype: fstype.to_string(),
                                total_bytes: usage.total,
                                used_bytes: usage.used,
                                free_bytes: usage.free,
                                used_percent: if usage.total > 0 {
                                    usage.used as f64 / usage.total as f64 * 100.0
                                } else { 0.0 },
                            });
                        }
                    }
                }
            }
        }

        // Aggregate totals
        for p in &self.disk.partitions {
            self.disk.total_bytes += p.total_bytes;
            self.disk.used_bytes += p.used_bytes;
            self.disk.free_bytes += p.free_bytes;
        }

        // Read disk I/O stats
        if let Ok(content) = fs::read_to_string("/proc/diskstats") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 14 {
                    let name = &parts[2];
                    if name.starts_with("sd") || name.starts_with("nvme") || name.starts_with("mmc") {
                        // Accumulate reads/writes
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

                    let mut speed = 0.0;
                    if let Ok(s) = fs::read_to_string(
                        format!("/sys/class/net/{}/speed", name)
                    ) {
                        speed = s.trim().parse().unwrap_or(0.0);
                    }

                    let mut ip = String::from("--");
                    // Read IP address from /proc/net/fib_trie or similar

                    self.network.interfaces.push(InterfaceInfo {
                        name: name.to_string(),
                        ip,
                        rx_bytes: rx,
                        tx_bytes: tx,
                        rx_speed: 0.0,
                        tx_speed: 0.0,
                        state: "up".to_string(),
                    });

                    self.network.total_rx_bytes += rx;
                    self.network.total_tx_bytes += tx;
                }
            }
        }
    }

    fn read_processes(&mut self) {
        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                let pid_str = entry.file_name();
                let pid: u32 = match pid_str.to_str().and_then(|s| s.parse().ok()) {
                    Some(p) => p,
                    None => continue,
                };

                let stat_path = entry.path().join("stat");
                let cmdline_path = entry.path().join("cmdline");
                let status_path = entry.path().join("status");

                if let Ok(content) = fs::read_to_string(&stat_path) {
                    let parts: Vec<&str> = content.split_whitespace().collect();
                    if parts.len() >= 13 {
                        let state = parts[2].to_string();
                        let name = parts[1].trim_matches('(').trim_matches(')').to_string();
                        let threads: u32 = parts.get(19).and_then(|s| s.parse().ok()).unwrap_or(0);

                        let mem_mb = if let Ok(status) = fs::read_to_string(&status_path) {
                            for line in status.lines() {
                                if line.starts_with("VmRSS:") {
                                    let val: f64 = line.split_whitespace()
                                        .nth(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                                    break val / 1024.0;
                                }
                            }
                        } else { 0.0 };

                        let user = std::process::Command::new("ps")
                            .args(["-o", "user=", "-p", &pid.to_string()])
                            .output()
                            .ok()
                            .and_then(|o| String::from_utf8(o.stdout).ok())
                            .unwrap_or_default()
                            .trim()
                            .to_string();

                        if self.processes.len() < 200 {
                            self.processes.push(ProcessInfo {
                                pid,
                                name,
                                cpu_percent: 0.0,
                                memory_mb: mem_mb,
                                state,
                                user,
                                threads,
                                fd_count: 0,
                                start_time: 0,
                            });
                        }
                    }
                }
            }
        }

        self.processes.sort_by(|a, b| b.memory_mb.partial_cmp(&a.memory_mb).unwrap());
        self.processes.truncate(100);
    }

    fn read_temperatures(&mut self) {
        for hwmon in 0..8 {
            let base = format!("/sys/class/hwmon/hwmon{}", hwmon);
            let name_path = format!("{}/name", base);
            if let Ok(name) = fs::read_to_string(&name_path) {
                for i in 1..8 {
                    let input_path = format!("{}/temp{}_input", base, i);
                    let max_path = format!("{}/temp{}_max", base, i);
                    let crit_path = format!("{}/temp{}_crit", base, i);

                    if let Ok(temp) = fs::read_to_string(&input_path) {
                        let temp_c = temp.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                        let max = fs::read_to_string(&max_path)
                            .ok()
                            .and_then(|s| s.trim().parse::<f64>().ok())
                            .map(|v| v / 1000.0)
                            .unwrap_or(100.0);
                        let crit = fs::read_to_string(&crit_path)
                            .ok()
                            .and_then(|s| s.trim().parse::<f64>().ok())
                            .map(|v| v / 1000.0)
                            .unwrap_or(120.0);

                        self.temps.push(Temperature {
                            name: format!("{} {}", name.trim(), i),
                            temp_c,
                            max_temp_c: max,
                            critical_temp_c: crit,
                        });
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
}

struct DiskUsage {
    total: u64,
    used: u64,
    free: u64,
}

fn fs_usage(path: &str) -> Option<DiskUsage> {
    use std::ffi::CString;
    use std::mem;

    let cpath = CString::new(path).ok()?;

    unsafe {
        let mut stat: libc::statvfs = mem::zeroed();
        if libc::statvfs(cpath.as_ptr(), &mut stat) == 0 {
            let block_size = stat.f_frsize as u64;
            let total = stat.f_blocks * block_size;
            let free = stat.f_bfree * block_size;
            let used = total - free;
            return Some(DiskUsage { total, used, free });
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

pub fn format_bytes_u64(bytes: u64) -> String {
    format_bytes(bytes as f64)
}
