use std::sync::Arc;
use std::time::Instant;
use parking_lot::RwLock;

pub struct DashboardWidgets {
    pub cpu: CpuWidget,
    pub memory: MemoryWidget,
    pub gpu: GpuWidget,
    pub disk: DiskWidget,
    pub network: NetworkWidget,
    pub ai: AIWidget,
    pub processes: ProcessWidget,
    pub timeline: TimelineWidget,
}

pub struct WidgetBase {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
    pub refresh_interval_ms: u32,
    last_refresh: Instant,
}

pub struct CpuWidget {
    pub base: WidgetBase,
    pub usage_percent: Arc<RwLock<Vec<f64>>>,
    pub per_core: Vec<f64>,
    pub temperature: f64,
    pub frequency: f64,
}

pub struct MemoryWidget {
    pub base: WidgetBase,
    pub percent_used: f64,
    pub used_gb: f64,
    pub total_gb: f64,
    pub swap_used_gb: f64,
    pub swap_total_gb: f64,
}

pub struct GpuWidget {
    pub base: WidgetBase,
    pub name: String,
    pub usage_percent: f64,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
    pub temperature: f64,
    pub power_watts: f64,
}

pub struct DiskWidget {
    pub base: WidgetBase,
    pub io_read: f64,
    pub io_write: f64,
    pub total_gb: f64,
    pub used_gb: f64,
}

pub struct NetworkWidget {
    pub base: WidgetBase,
    pub rx_speed: f64,
    pub tx_speed: f64,
    pub total_rx_gb: f64,
    pub total_tx_gb: f64,
    pub connections: u32,
}

pub struct AIWidget {
    pub base: WidgetBase,
    pub status: AIStatus,
    pub memory_nodes: u64,
    pub memory_edges: u64,
    pub active_agents: u32,
    pub tasks_queued: u32,
    pub automations: u32,
    pub learning_active: bool,
    pub last_query: String,
    pub confidence: f32,
}

pub enum AIStatus {
    Initializing,
    Active,
    Processing,
    Idle,
    Error(String),
}

pub struct ProcessWidget {
    pub base: WidgetBase,
    pub processes: Vec<ProcessEntry>,
}

pub struct ProcessEntry {
    pub pid: u32,
    pub name: String,
    pub cpu: f64,
    pub memory_mb: f64,
    pub state: String,
}

pub struct TimelineWidget {
    pub base: WidgetBase,
    pub events: Vec<TimelineEvent>,
}

pub struct TimelineEvent {
    pub time: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub description: String,
    pub severity: EventSeverity,
}

pub enum EventSeverity {
    Info,
    Warning,
    Error,
    Success,
}
