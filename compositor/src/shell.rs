use crate::config::CompositorConfig;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct ShellIntegration {
    config: Arc<RwLock<CompositorConfig>>,
    panel_height: i32,
    launcher_open: bool,
    notification_panel_open: bool,
}

impl ShellIntegration {
    pub fn new(config: Arc<RwLock<CompositorConfig>>) -> Self {
        Self {
            config,
            panel_height: 48,
            launcher_open: false,
            notification_panel_open: false,
        }
    }

    pub fn toggle_launcher(&mut self) {
        self.launcher_open = !self.launcher_open;
    }

    pub fn toggle_notifications(&mut self) {
        self.notification_panel_open = !self.notification_panel_open;
    }

    pub fn panel_height(&self) -> i32 {
        self.panel_height
    }

    pub fn is_launcher_open(&self) -> bool {
        self.launcher_open
    }

    pub fn is_notification_panel_open(&self) -> bool {
        self.notification_panel_open
    }
}
