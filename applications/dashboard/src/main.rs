mod widgets;
mod monitor;
mod render;

use monitor::SystemMonitor;
use render::DashboardRenderer;
use std::time::{Duration, Instant};
use std::thread;

fn main() -> anyhow::Result<()> {
    println!("\x1b]0;Prometheus Dashboard\x07");
    println!("\x1b[38;2;0;120;255m╔══════════════════════════════════════════════════════════════════╗");
    println!("║               PROMETHEUS OS — AI COMMAND CENTER               ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\x1b[0m");
    println!("\x1b[2m  Live system intelligence dashboard | v{}\x1b[0m\n", env!("CARGO_PKG_VERSION"));

    let mut monitor = SystemMonitor::new();
    let renderer = DashboardRenderer::new();
    let mut last_refresh = Instant::now();

    loop {
        let now = Instant::now();
        if now.duration_since(last_refresh) >= Duration::from_secs(2) {
            monitor.refresh_all();
            renderer.display(&monitor);
            last_refresh = now;
        }

        thread::sleep(Duration::from_millis(200));

        #[cfg(unix)]
        unsafe {
            let mut buf = [0u8; 1];
            let flags = libc::fcntl(0, libc::F_GETFL, 0);
            libc::fcntl(0, libc::F_SETFL, flags | libc::O_NONBLOCK);
            if libc::read(0, buf.as_mut_ptr() as *mut _, 1) > 0 && buf[0] == b'q' {
                break;
            }
            libc::fcntl(0, libc::F_SETFL, flags);
        }
    }

    Ok(())
}
