use std::fs;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or(".");

    println!("\x1b[38;2;0;120;255mPrometheus Files v{}\x1b[0m", env!("CARGO_PKG_VERSION"));
    println!("Path: {}\n", path);

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy().to_string();
        let path = entry.path();

        if path.is_dir() {
            println!("\x1b[34m📁 {}/\x1b[0m", name);
        } else {
            let metadata = fs::metadata(&path)?;
            let size = metadata.len();
            let size_str = if size > 1_000_000_000 {
                format!("{:.1}GB", size as f64 / 1_000_000_000.0)
            } else if size > 1_000_000 {
                format!("{:.1}MB", size as f64 / 1_000_000.0)
            } else if size > 1_000 {
                format!("{:.1}KB", size as f64 / 1_000.0)
            } else {
                format!("{}B", size)
            };
            println!("  {:48} {}", name, size_str);
        }
    }

    Ok(())
}
