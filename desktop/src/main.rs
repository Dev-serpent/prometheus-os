mod panel;
mod launcher;
mod notification;

use panel::PrometheusPanel;
use launcher::PrometheusLauncher;
use notification::NotificationCenter;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("prometheus_desktop=info")
        .init();

    tracing::info!("Prometheus Desktop v{}", env!("CARGO_PKG_VERSION"));

    let panel = PrometheusPanel::new();
    let launcher = PrometheusLauncher::new();
    let notifications = NotificationCenter::new();

    panel.run()?;
    launcher.run()?;
    notifications.run()?;

    Ok(())
}
