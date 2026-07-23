use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let sm = prometheus_snapshot_manager::SnapshotManager::new();

    if args.len() < 2 {
        println!("Prometheus Snapshot Manager v{}", env!("CARGO_PKG_VERSION"));
        println!("Btrfs snapshot management with AI-assisted recovery");
        println!();
        println!("Usage: prometheus-snapshot-manager <command> [args]");
        println!("Commands: list, create, restore, delete, status");
        return Ok(());
    }

    match args[1].as_str() {
        "list" => {
            let snapshots = sm.list_snapshots()?;
            for snap in &snapshots {
                println!("{:>6}  {}  {}  {}", snap.number, snap.date, snap.type_, snap.cleanup);
            }
        }
        "create" => {
            let desc = args.get(2).map(|s| s.as_str()).unwrap_or("manual");
            let snap = sm.create_snapshot(desc)?;
            println!("Created snapshot #{}: {}", snap.number, snap.description);
        }
        "restore" => {
            if let Some(num) = args.get(2).and_then(|s| s.parse::<u64>().ok()) {
                sm.restore_snapshot(num)?;
                println!("Restored snapshot #{}", num);
            }
        }
        "delete" => {
            if let Some(num) = args.get(2).and_then(|s| s.parse::<u64>().ok()) {
                sm.delete_snapshot(num)?;
                println!("Deleted snapshot #{}", num);
            }
        }
        "status" => {
            let status = sm.get_status()?;
            println!("Btrfs Status:");
            println!("  Used: {} bytes", status.used_bytes);
            println!("  Free: {} bytes", status.free_bytes);
            println!("  Snapshots: {}", status.snapshot_count);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }
    Ok(())
}
