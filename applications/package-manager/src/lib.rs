use std::process::Command;

pub struct PackageManager {
    pacman: PacmanBackend,
    aur: AURBackend,
    flatpak: FlatpakBackend,
}

struct PacmanBackend;
struct AURBackend;
struct FlatpakBackend;

pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub size: u64,
    pub installed: bool,
    pub repository: String,
    pub dependencies: Vec<String>,
}

pub struct PackageUpdate {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub size: u64,
}

impl PackageManager {
    pub fn new() -> Self {
        Self {
            pacman: PacmanBackend,
            aur: AURBackend,
            flatpak: FlatpakBackend,
        }
    }

    pub fn search(&self, query: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();
        packages.extend(self.search_pacman(query));
        packages.extend(self.search_aur(query));
        packages.extend(self.search_flatpak(query));
        packages
    }

    fn search_pacman(&self, query: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();
        if let Ok(output) = Command::new("pacman")
            .args(["-Ss", query])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains('/') {
                    let parts: Vec<&str> = line.splitn(2, ' ').collect();
                    if parts.len() >= 2 {
                        let repo_pkg = parts[0];
                        let desc = parts.get(1).unwrap_or(&"");
                        let repo_parts: Vec<&str> = repo_pkg.splitn(2, '/').collect();
                        if repo_parts.len() == 2 {
                            packages.push(PackageInfo {
                                name: repo_parts[1].to_string(),
                                version: String::new(),
                                description: desc.to_string(),
                                size: 0,
                                installed: false,
                                repository: repo_parts[0].to_string(),
                                dependencies: Vec::new(),
                            });
                        }
                    }
                }
            }
        }
        packages
    }

    fn search_aur(&self, query: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        // Try yay first, then paru
        for helper in &["yay", "paru"] {
            if let Ok(output) = Command::new(helper)
                .args(["-Ss", query])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.starts_with("aur/") {
                        let name = line.trim_start_matches("aur/");
                        let pkg_name = name.splitn(2, ' ').next().unwrap_or("");
                        packages.push(PackageInfo {
                            name: pkg_name.to_string(),
                            version: String::new(),
                            description: String::new(),
                            size: 0,
                            installed: false,
                            repository: "aur".to_string(),
                            dependencies: Vec::new(),
                        });
                    }
                }
                if !packages.is_empty() {
                    break;
                }
            }
        }
        packages
    }

    fn search_flatpak(&self, query: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();
        if let Ok(output) = Command::new("flatpak")
            .args(["search", query])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.splitn(4, '\t').collect();
                if parts.len() >= 2 {
                    packages.push(PackageInfo {
                        name: parts[0].to_string(),
                        version: parts.get(1).unwrap_or(&"").to_string(),
                        description: parts.get(2).unwrap_or(&"").to_string(),
                        size: 0,
                        installed: false,
                        repository: "flatpak".to_string(),
                        dependencies: Vec::new(),
                    });
                }
            }
        }
        packages
    }

    pub fn install(&self, package: &str) -> anyhow::Result<()> {
        let status = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", package])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to install package: {}", package);
        }
        Ok(())
    }

    pub fn remove(&self, package: &str) -> anyhow::Result<()> {
        let status = Command::new("sudo")
            .args(["pacman", "-R", "--noconfirm", package])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to remove package: {}", package);
        }
        Ok(())
    }

    pub fn update_all(&self) -> anyhow::Result<Vec<PackageUpdate>> {
        let mut updates = Vec::new();

        // Pacman updates
        if let Ok(output) = Command::new("pacman")
            .args(["-Qu"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    updates.push(PackageUpdate {
                        name: parts[0].to_string(),
                        current_version: parts[1].trim_end_matches("->").to_string(),
                        new_version: parts.get(2).unwrap_or(&"unknown").to_string(),
                        size: 0,
                    });
                }
            }
        }

        // AUR updates via yay/paru
        for helper in &["yay", "paru"] {
            if let Ok(output) = Command::new(helper)
                .args(["-Qua"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        updates.push(PackageUpdate {
                            name: parts[0].trim_start_matches("aur/").to_string(),
                            current_version: parts[1].trim_end_matches("->").to_string(),
                            new_version: parts.get(2).unwrap_or(&"unknown").to_string(),
                            size: 0,
                        });
                    }
                }
                break;
            }
        }

        Ok(updates)
    }

    pub fn system_upgrade(&self) -> anyhow::Result<()> {
        let status = Command::new("sudo")
            .args(["pacman", "-Syu", "--noconfirm"])
            .status()?;

        if !status.success() {
            anyhow::bail!("System upgrade failed");
        }

        // Also try AUR upgrade
        for helper in &["yay", "paru"] {
            let _ = Command::new(helper)
                .args(["-Sua", "--noconfirm"])
                .status();
            break;
        }

        Ok(())
    }

    pub fn list_installed(&self) -> Vec<PackageInfo> {
        let mut packages = Vec::new();
        if let Ok(output) = Command::new("pacman")
            .args(["-Q"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    packages.push(PackageInfo {
                        name: parts[0].to_string(),
                        version: parts[1].to_string(),
                        description: String::new(),
                        size: 0,
                        installed: true,
                        repository: String::new(),
                        dependencies: Vec::new(),
                    });
                }
            }
        }
        packages
    }

    pub fn check_cache(&self) -> anyhow::Result<u64> {
        let output = Command::new("du")
            .args(["-sb", "/var/cache/pacman/pkg"])
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let size = stdout.split_whitespace().next()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        Ok(size)
    }

    pub fn clean_cache(&self) -> anyhow::Result<()> {
        let status = Command::new("sudo")
            .args(["pacman", "-Sc", "--noconfirm"])
            .status()?;
        if !status.success() {
            anyhow::bail!("Failed to clean cache");
        }
        Ok(())
    }
}
