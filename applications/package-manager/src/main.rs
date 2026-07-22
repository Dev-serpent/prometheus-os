fn main() -> anyhow::Result<()> {
    println!("Prometheus Package Manager v{}", env!("CARGO_PKG_VERSION"));
    println!("AI-assisted package management with pacman, AUR, and Flatpak");
    Ok(())
}
