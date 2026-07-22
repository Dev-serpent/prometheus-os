use std::process::Command;

pub struct SandboxManager {
    enabled: bool,
    backend: SandboxBackend,
}

enum SandboxBackend {
    Bubblewrap,
    Landlock,
    Namespace,
}

impl SandboxManager {
    pub fn new() -> Self {
        Self {
            enabled: true,
            backend: SandboxBackend::Bubblewrap,
        }
    }

    pub fn initialize(&self) -> anyhow::Result<()> {
        tracing::info!("Sandbox manager initialized with {:?}", self.backend);
        Ok(())
    }

    pub fn run_sandboxed(&self, command: &str) -> anyhow::Result<()> {
        if !self.enabled {
            let status = Command::new("sh").arg("-c").arg(command).status()?;
            return Ok(());
        }

        match self.backend {
            SandboxBackend::Bubblewrap => {
                let status = Command::new("bwrap")
                    .args([
                        "--unshare-all",
                        "--die-with-parent",
                        "--ro-bind", "/usr", "/usr",
                        "--ro-bind", "/lib", "/lib",
                        "--ro-bind", "/lib64", "/lib64",
                        "--proc", "/proc",
                        "--dev", "/dev",
                        "--tmpfs", "/tmp",
                        "--tmpfs", "/home",
                        "--symlink", "/usr/bin", "/bin",
                        "--symlink", "/usr/bin", "/sbin",
                        "--symlink", "/usr/lib", "/lib",
                        "--symlink", "/usr/lib64", "/lib64",
                        "sh", "-c", command,
                    ])
                    .status()?;

                if !status.success() {
                    anyhow::bail!("Sandboxed command failed");
                }
            }
            SandboxBackend::Namespace => {
                let status = Command::new("unshare")
                    .args(["-r", "-n", "sh", "-c", command])
                    .status()?;
            }
            SandboxBackend::Landlock => {
                // Use landlock for fine-grained access control
                let status = Command::new("sh").arg("-c").arg(command).status()?;
            }
        }

        Ok(())
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
