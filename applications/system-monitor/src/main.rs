mod monitor;

use monitor::SystemMonitor;
use std::time::{Duration, Instant};
use std::thread;

fn main() -> anyhow::Result<()> {
    println!("\x1b[38;2;0;120;255m╔══════════════════════════════════════════════════════════════╗");
    println!("║            PROMETHEUS OS SYSTEM MONITOR v{}          ║", env!("CARGO_PKG_VERSION"));
    println!("╚══════════════════════════════════════════════════════════════╝\x1b[0m");

    let mut monitor = SystemMonitor::new();
    let mut last_refresh = Instant::now();

    loop {
        let now = Instant::now();
        if now.duration_since(last_refresh) >= Duration::from_secs(1) {
            monitor.refresh_all();
            render_display(&monitor);
            last_refresh = now;
        }

        thread::sleep(Duration::from_millis(100));

        // Check for 'q' key to quit (in real terminal mode)
        #[cfg(unix)]
        unsafe {
            use std::os::unix::io::RawFd;
            let mut buf = [0u8; 1];
            let flags = libc::fcntl(0, libc::F_GETFL, 0);
            libc::fcntl(0, libc::F_SETFL, flags | libc::O_NONBLOCK);
            if libc::read(0, buf.as_mut_ptr() as *mut _, 1) > 0 {
                if buf[0] == b'q' {
                    break;
                }
            }
            libc::fcntl(0, libc::F_SETFL, flags);
        }
    }

    Ok(())
}

fn render_display(m: &SystemMonitor) {
    // Clear screen
    print!("\x1b[2J\x1b[H");

    let cpu = m.cpu();
    let mem = m.memory();
    let gpu = m.gpu();
    let disk = m.disk();

    // === CPU Section ===
    println!("\x1b[38;2;0;120;255m━━━ CPU ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    print_bar("Total", cpu.usage_percent, cpu_color);
    print_bar("IOWait", cpu.iowait_percent, |v| if v > 10.0 { color_red(v) } else { color_green(v) });
    println!("  Temperature: \x1b[{}m{:.0}°C\x1b[0m  Frequency: {:.0} MHz  Cores: {}",
        temp_color(cpu.temperature_c), cpu.temperature_c, cpu.frequency_mhz, cpu.core_count);
    println!("  Load Average: {:.2} {:.2} {:.2}", cpu.load_1, cpu.load_5, cpu.load_15);

    // === Memory Section ===
    println!("\n\x1b[38;2;0;120;255m━━━ MEMORY ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    print_bar("RAM", mem.percent_used, mem_color);
    println!("  {} / {} used  |  {} available  |  {} cached",
        color_bytes(mem.used_kb, mem.total_kb),
        color_bytes(mem.total_kb, mem.total_kb),
        color_bytes(mem.available_kb, mem.total_kb),
        color_bytes(mem.cached_kb, mem.total_kb));
    println!("  Swap: {} / {}",
        color_bytes(mem.swap_used_kb, mem.swap_total_kb),
        color_bytes(mem.swap_total_kb, mem.swap_total_kb));

    // === GPU Section ===
    println!("\n\x1b[38;2;0;120;255m━━━ GPU ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    if !gpu.name.is_empty() {
        println!("  \x1b[1m{}\x1b[0m", gpu.name);
        print_bar("Usage", gpu.usage_percent, gpu_color);
        println!("  VRAM: {} / {}  |  Temp: \x1b[{}m{:.0}°C\x1b[0m  |  Power: {:.1}W",
            color_bytes(gpu.vram_used_mb * 1024, gpu.vram_total_mb * 1024),
            color_bytes(gpu.vram_total_mb * 1024, gpu.vram_total_mb * 1024),
            temp_color(gpu.temperature_c), gpu.temperature_c, gpu.power_watts);
    } else {
        println!("  \x1b[3mNo GPU detected or GPU monitoring not available\x1b[0m");
    }

    // === Disk Section ===
    println!("\n\x1b[38;2;0;120;255m━━━ DISK ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    for p in &disk.partitions {
        if p.mount == "/" || p.mount == "/home" {
            print_bar(&p.mount, p.used_percent, disk_color);
            println!("   {} / {} ({})",
                color_bytes(p.used_bytes, p.total_bytes),
                color_bytes(p.total_bytes, p.total_bytes),
                p.fstype);
        }
    }

    // === Network Section ===
    println!("\n\x1b[38;2;0;120;255m━━━ NETWORK ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    for iface in &m.network().interfaces {
        println!("  {:<8} ↓ {}  ↑ {}",
            iface.name,
            iface.rx_bytes, iface.tx_bytes);
    }

    // === Processes Section ===
    println!("\n\x1b[38;2;0;120;255m━━━ TOP PROCESSES ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    println!("  {:<8} {:<20} {:>8} {:>8}", "PID", "NAME", "CPU%", "MEM(MB)");
    for p in m.processes().iter().take(10) {
        println!("  {:<8} {:<20} {:>7.1}% {:>8.0}",
            p.pid, truncate(&p.name, 20), p.cpu_percent, p.memory_mb);
    }

    // === Temperatures Section ===
    if !m.temperatures().is_empty() {
        println!("\n\x1b[38;2;0;120;255m━━━ TEMPERATURES ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
        for t in &m.temperatures[..3.min(m.temperatures().len())] {
            println!("  {:<20} \x1b[{}m{:.0}°C\x1b[0m / {:.0}°C",
                t.name, temp_color(t.temp_c), t.temp_c, t.max_temp_c);
        }
    }

    println!("\n\x1b[38;2;0;120;255m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    print!("\x1b[2mPress 'q' to quit\x1b[0m");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

fn print_bar(label: &str, percent: f64, color_fn: fn(f64) -> String) {
    let width = 40;
    let filled = ((percent / 100.0) * width as f64).round() as usize;
    let filled = filled.min(width);
    let empty = width - filled;

    let bar = format!("{}{}",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(empty));

    println!("  {:<12} \x1b[{}m{}\x1b[0m {:>6.1}%",
        label, color_fn(percent), bar, percent);
}

fn cpu_color(v: f64) -> String {
    if v > 80.0 { "38;2;255;23;68".to_string()
    } else if v > 50.0 { "38;2;255;214;0".to_string()
    } else { "38;2;0;200;83".to_string() }
}

fn mem_color(v: f64) -> String {
    if v > 90.0 { "38;2;255;23;68".to_string()
    } else if v > 70.0 { "38;2;255;214;0".to_string()
    } else { "38;2;0;120;255".to_string() }
}

fn gpu_color(v: f64) -> String {
    if v > 90.0 { "38;2;255;23;68".to_string()
    } else if v > 60.0 { "38;2;255;214;0".to_string()
    } else { "38;2;0;200;83".to_string() }
}

fn disk_color(v: f64) -> String {
    if v > 90.0 { "38;2;255;23;68".to_string()
    } else if v > 70.0 { "38;2;255;214;0".to_string()
    } else { "38;2;0;120;255".to_string() }
}

fn color_green(_v: f64) -> String { "38;2;0;200;83".to_string() }
fn color_red(_v: f64) -> String { "38;2;255;23;68".to_string() }

fn temp_color(temp: f64) -> String {
    if temp > 80.0 { "38;2;255;23;68".to_string()
    } else if temp > 60.0 { "38;2;255;214;0".to_string()
    } else { "38;2;0;200;83".to_string() }
}

fn color_bytes(val: u64, total: u64) -> String {
    let pct = if total > 0 { val as f64 / total as f64 * 100.0 } else { 0.0 };
    let color = if pct > 90.0 { "255;23;68" } else if pct > 70.0 { "255;214;0" } else { "0;200;83" };
    format!("\x1b[38;2;{}m{}\x1b[0m", color, monitor::format_bytes_u64(val))
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { s.to_string() }
    else { format!("{}…", &s[..max-1]) }
}


