fn main() -> anyhow::Result<()> {
    println!("Prometheus System Monitor v{}", env!("CARGO_PKG_VERSION"));
    println!("Real-time system resource monitoring");
    Ok(())
}
