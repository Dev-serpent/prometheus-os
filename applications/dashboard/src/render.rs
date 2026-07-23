use crate::monitor::{SystemMonitor, format_bytes};
use std::time::Duration;

pub struct DashboardRenderer;

impl DashboardRenderer {
    pub fn new() -> Self { Self }

    pub fn clear(&self) {
        print!("\x1b[2J\x1b[H");
    }

    fn bar(&self, label: &str, pct: f64, color: &str, width: usize) {
        let filled = ((pct / 100.0) * width as f64).min(width as f64) as usize;
        let empty = width.saturating_sub(filled);
        print!(" {} {} ", label, color);
        for _ in 0..filled { print!("█"); }
        for _ in 0..empty { print!("░"); }
        print!("\x1b[0m {:5.1}%\n", pct);
    }

    pub fn display(&self, m: &SystemMonitor, elapsed: Duration) {
        print!("\x1b[2J\x1b[H\x1b[?25l");

        // Header
        let ver = env!("CARGO_PKG_VERSION");
        let uptime = elapsed.as_secs();
        let h = uptime / 3600;
        let m_ = (uptime % 3600) / 60;
        let s = uptime % 60;
        println!("\x1b[38;2;0;120;255m┌──────────────────────────────────────────────────────────────────────────────┐");
        println!("│  \x1b[1mPROMETHEUS OS\x1b[0m\x1b[38;2;0;120;255m  —  AI Command Center  v{}  │  Uptime {:02}:{:02}:{:02}  │", ver, h, m_, s);
        println!("└──────────────────────────────────────────────────────────────────────────────┘\x1b[0m");

        // CPU
        let cpu = m.cpu();
        let c = if cpu.usage_percent > 80.0 { "\x1b[38;2;255;80;80m" }
                else if cpu.usage_percent > 50.0 { "\x1b[38;2;255;200;50m" }
                else { "\x1b[38;2;0;200;100m" };
        println!("\n\x1b[38;2;0;120;255m CPU\x1b[0m  {} cores  │  {}°C  │  {} MHz",
            cpu.core_count, cpu.temperature_c as u64, cpu.frequency_mhz as u64);
        self.bar("     ", cpu.usage_percent, c, 50);

        // Memory
        let mem = m.memory();
        let mem_pct = if mem.total_kb > 0 { mem.used_kb as f64 / mem.total_kb as f64 * 100.0 } else { 0.0 };
        let c = if mem_pct > 80.0 { "\x1b[38;2;255;80;80m" }
                else if mem_pct > 50.0 { "\x1b[38;2;255;200;50m" }
                else { "\x1b[38;2;0;200;100m" };
        println!("\n\x1b[38;2;0;120;255m RAM\x1b[0m  {} / {}  │  Swap: {} / {}",
            format_bytes(mem.used_kb as f64 * 1024.0),
            format_bytes(mem.total_kb as f64 * 1024.0),
            format_bytes(mem.swap_used_kb as f64 * 1024.0),
            format_bytes(mem.swap_total_kb as f64 * 1024.0));
        self.bar("     ", mem_pct, c, 50);

        // GPU
        let gpu = m.gpu();
        if !gpu.name.is_empty() {
            let c = if gpu.usage_percent > 80.0 { "\x1b[38;2;255;80;80m" }
                    else if gpu.usage_percent > 50.0 { "\x1b[38;2;255;200;50m" }
                    else { "\x1b[38;2;0;200;100m" };
            println!("\n\x1b[38;2;0;120;255m GPU\x1b[0m  {}  │  {}°C  │  VRAM {} / {}",
                gpu.name, gpu.temperature_c as u64,
                format_bytes(gpu.vram_used_mb as f64 * 1024.0 * 1024.0),
                format_bytes(gpu.vram_total_mb as f64 * 1024.0 * 1024.0));
            self.bar("     ", gpu.usage_percent, c, 50);
        }

        // Disk
        let disk = m.disk();
        let disk_pct = if disk.total_bytes > 0 { disk.used_bytes as f64 / disk.total_bytes as f64 * 100.0 } else { 0.0 };
        let c = if disk_pct > 80.0 { "\x1b[38;2;255;80;80m" }
                else if disk_pct > 50.0 { "\x1b[38;2;255;200;50m" }
                else { "\x1b[38;2;0;200;100m" };
        println!("\n\x1b[38;2;0;120;255m DISK\x1b[0m  {} / {}  │  R: {}/s  W: {}/s",
            format_bytes(disk.used_bytes as f64),
            format_bytes(disk.total_bytes as f64),
            format_bytes(disk.read_bytes_per_sec),
            format_bytes(disk.write_bytes_per_sec));
        self.bar("     ", disk_pct, c, 50);

        // Network
        let net = m.network();
        println!("\n\x1b[38;2;0;120;255m NET\x1b[0m  ↓ {}/s  ↑ {}/s  │  {} connections",
            format_bytes(net.rx_speed), format_bytes(net.tx_speed), net.connections);

        // AI Activity
        let ai = m.ai();
        println!("\n\x1b[38;2;0;120;255m AI\x1b[0m  {} agents  │  {} memory nodes  │  {} tasks queued",
            ai.active_agents, ai.memory_nodes, ai.tasks_queued);

        // Processes
        println!("\n\x1b[38;2;0;120;255m TOP\x1b[0m");
        let procs = m.processes();
        for (i, p) in procs.iter().take(8).enumerate() {
            println!(" {:>2}. {:<20} {:>6.1}% {:>7}K", i + 1,
                if p.name.len() > 18 { &p.name[..18] } else { &p.name },
                p.cpu_usage, p.memory_kb / 1024);
        }

        // Temperatures
        let temps = m.temperatures();
        if !temps.is_empty() {
            print!("\n\x1b[38;2;0;120;255m TEMP\x1b[0m");
            for t in temps {
                let c = if t.temp_c > 80.0 { "\x1b[38;2;255;80;80m" }
                        else if t.temp_c > 60.0 { "\x1b[38;2;255;200;50m" }
                        else { "\x1b[38;2;0;200;100m" };
                print!("  {}{:.0}°C\x1b[0m", c, t.temp_c);
            }
            println!();
        }

        println!("\n\x1b[2m  Press 'q' to quit  │  Auto-refresh every 1s\x1b[0m");
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
}
