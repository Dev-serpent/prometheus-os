use std::collections::HashMap;

pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
    enabled: HashMap<String, bool>,
    registry: PluginRegistry,
}

pub trait AIPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn capabilities(&self) -> Vec<Capability>;
    fn execute(&self, input: &str) -> Result<String, PluginError>;
}

pub enum Capability {
    Knowledge,
    Computation,
    WebSearch,
    FileAccess,
    CodeExecution,
    ImageGeneration,
    DataAnalysis,
    Communication,
}

pub enum PluginError {
    NotLoaded,
    ExecutionFailed(String),
    PermissionDenied,
    Timeout,
}

struct Plugin {
    instance: Box<dyn AIPlugin>,
    permissions: Vec<String>,
    loaded_at: chrono::DateTime<chrono::Utc>,
}

struct PluginRegistry {
    python: Vec<String>,
    rust: Vec<String>,
    native: Vec<String>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            enabled: HashMap::new(),
            registry: PluginRegistry {
                python: Vec::new(),
                rust: Vec::new(),
                native: Vec::new(),
            },
        }
    }

    pub fn load_plugin(&mut self, plugin: Box<dyn AIPlugin>) {
        let name = plugin.name().to_string();
        self.plugins.insert(
            name.clone(),
            Plugin {
                instance: plugin,
                permissions: Vec::new(),
                loaded_at: chrono::Utc::now(),
            },
        );
        self.enabled.insert(name, true);
    }

    pub fn execute(&self, name: &str, input: &str) -> Result<String, PluginError> {
        match self.plugins.get(name) {
            Some(plugin) if *self.enabled.get(name).unwrap_or(&false) => {
                plugin.instance.execute(input)
            }
            Some(_) => Err(PluginError::PermissionDenied),
            None => Err(PluginError::NotLoaded),
        }
    }

    pub fn enable(&mut self, name: &str) {
        self.enabled.insert(name.to_string(), true);
    }

    pub fn disable(&mut self, name: &str) {
        self.enabled.insert(name.to_string(), false);
    }
}
