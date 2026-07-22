fn main() -> anyhow::Result<()> {
    println!("Prometheus Plugin Manager v{}", env!("CARGO_PKG_VERSION"));
    println("Extend Prometheus OS with plugins");
    Ok(())
}
