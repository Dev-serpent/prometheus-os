fn main() -> anyhow::Result<()> {
    println!("Prometheus Task Manager v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
