mod monitor;

use monitor::SystemMonitor;

fn main() -> anyhow::Result<()> {
    println!("Prometheus Dashboard v{}", env!("CARGO_PKG_VERSION"));
    println!("AI command center and system overview");

    let monitor = SystemMonitor::new();
    monitor.display();

    Ok(())
}
