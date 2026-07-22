mod compositor;
mod config;
mod layout;
mod effects;
mod input;
mod workspace;
mod shell;
mod render;

use compositor::PrometheusCompositor;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    tracing::info!("Prometheus Compositor v{}", env!("CARGO_PKG_VERSION"));

    let mut compositor = PrometheusCompositor::new()?;
    compositor.run()?;

    Ok(())
}
