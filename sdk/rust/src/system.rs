pub struct SystemAPI;

impl SystemAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn execute_command(&self, command: &str) -> anyhow::Result<String> {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn memory_info(&self) -> anyhow::Result<MemoryInfo> {
        Ok(MemoryInfo {
            total: 0,
            used: 0,
            free: 0,
        })
    }

    pub fn cpu_info(&self) -> anyhow::Result<CpuInfo> {
        Ok(CpuInfo {
            usage: 0.0,
            cores: 0,
        })
    }
}

pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

pub struct CpuInfo {
    pub usage: f32,
    pub cores: u32,
}
