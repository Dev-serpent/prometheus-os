use crate::memory::MemoryGraph;
use std::sync::Arc;

pub struct ContextManager {
    memory: Arc<MemoryGraph>,
    current: ContextState,
    history: Vec<ContextSnapshot>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextState {
    pub active_window: Option<String>,
    pub active_app: Option<String>,
    pub current_workspace: u32,
    pub foreground_process: Option<String>,
    pub clipboard: Option<String>,
    pub battery_percent: f32,
    pub network_connected: bool,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub recent_events: Vec<String>,
    pub time: String,
    pub user_idle: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ContextSnapshot {
    timestamp: i64,
    state: ContextState,
}

impl ContextManager {
    pub fn new(memory: Arc<MemoryGraph>) -> Self {
        Self {
            memory,
            current: ContextState {
                active_window: None,
                active_app: None,
                current_workspace: 0,
                foreground_process: None,
                clipboard: None,
                battery_percent: 100.0,
                network_connected: true,
                cpu_usage: 0.0,
                memory_usage: 0.0,
                recent_events: Vec::new(),
                time: String::new(),
                user_idle: false,
            },
            history: Vec::new(),
        }
    }

    pub fn initialize(&self) {
        tracing::info!("Context manager initialized");
    }

    pub fn update(&mut self) {
        let snapshot = ContextSnapshot {
            timestamp: chrono::Utc::now().timestamp(),
            state: self.current.clone(),
        };

        self.history.push(snapshot);

        // Trim history to last 1000 snapshots
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    pub fn current_state(&self) -> ContextState {
        self.current.clone()
    }

    pub fn recent_history(&self, count: usize) -> Vec<ContextState> {
        self.history
            .iter()
            .rev()
            .take(count)
            .map(|s| s.state.clone())
            .collect()
    }
}
