mod widgets;
mod monitor;
mod render;

use monitor::SystemMonitor;
use render::DashboardRenderer;
use std::time::{Duration, Instant};
use std::thread;

fn main() -> anyhow::Result<()> {
    println!("\x1b]0;Prometheus OS — AI Command Center\x07");

    let mut monitor = SystemMonitor::new();
    let renderer = DashboardRenderer::new();
    let mut last_refresh = Instant::now();
    let start = Instant::now();

    loop {
        let now = Instant::now();
        if now.duration_since(last_refresh) >= Duration::from_secs(1) {
            monitor.refresh_all();
            renderer.display(&monitor, start.elapsed());
            last_refresh = now;
        }

        thread::sleep(Duration::from_millis(100));

        #[cfg(unix)]
        unsafe {
            let mut buf = [0u8; 1];
            let flags = libc::fcntl(0, libc::F_GETFL, 0);
            libc::fcntl(0, libc::F_SETFL, flags | libc::O_NONBLOCK);
            if libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 1) > 0 && buf[0] == b'q' {
                break;
            }
            libc::fcntl(0, libc::F_SETFL, flags);
        }
    }

    renderer.clear();
    println!("\x1b[38;2;0;120;255mPrometheus OS\x1b[0m — session ended after {:.1}s", start.elapsed().as_secs_f64());
    Ok(())
}
