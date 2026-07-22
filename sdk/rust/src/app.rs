use serde::{Deserialize, Serialize};

pub struct AppFramework {
    initialized: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub icon: String,
    pub window_config: WindowConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub decorations: bool,
    pub transparency: bool,
}

impl AppFramework {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    pub fn configure(&mut self, _config: AppConfig) {
        self.initialized = true;
    }

    pub fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
