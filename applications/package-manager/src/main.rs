use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let pm = prometheus_package_manager::PackageManager::new();

    if args.len() < 2 {
        println!("Prometheus Package Manager v{}", env!("CARGO_PKG_VERSION"));
        println!("AI-assisted package management with pacman, AUR, and Flatpak");
        println!();
        println!("Usage: prometheus-package-manager <command> [args]");
        println!("Commands: search, install, remove, update, list, clean, upgrade");
        return Ok(());
    }

    match args[1].as_str() {
        "search" => {
            let query = args.get(2).map(|s| s.as_str()).unwrap_or("");
            let results = pm.search(query);
            for pkg in &results {
                println!("{} ({}) - {}", pkg.name, pkg.repository, pkg.description);
            }
            if results.is_empty() {
                println!("No packages found for '{}'", query);
            }
        }
        "install" => {
            if let Some(pkg) = args.get(2) {
                pm.install(pkg)?;
                println!("Installed: {}", pkg);
            }
        }
        "remove" => {
            if let Some(pkg) = args.get(2) {
                pm.remove(pkg)?;
                println!("Removed: {}", pkg);
            }
        }
        "list" => {
            let installed = pm.list_installed();
            for pkg in &installed {
                println!("{} {}", pkg.name, pkg.version);
            }
        }
        "update" => {
            let updates = pm.update_all()?;
            for u in &updates {
                println!("{} {} -> {}", u.name, u.current_version, u.new_version);
            }
            if updates.is_empty() {
                println!("System is up to date.");
            }
        }
        "upgrade" => {
            pm.system_upgrade()?;
            println!("System upgraded successfully.");
        }
        "clean" => {
            let size = pm.check_cache()?;
            pm.clean_cache()?;
            println!("Cleaned {} bytes of package cache.", size);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }
    Ok(())
}
