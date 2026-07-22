use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct AuditLogger {
    enabled: bool,
    log_path: PathBuf,
    buffer: Vec<AuditEntry>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditEntry {
    timestamp: i64,
    action: String,
    user: String,
    result: String,
    details: Option<String>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            enabled: true,
            log_path: PathBuf::from("/var/log/prometheus/audit.log"),
            buffer: Vec::with_capacity(100),
        }
    }

    pub fn initialize(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.log_path.parent() {
            fs::create_dir_all(parent)?;
        }
        tracing::info!("Audit logger initialized");
        Ok(())
    }

    pub fn log(&self, action: &str, user: &str, result: &str) {
        if !self.enabled {
            return;
        }

        let entry = AuditEntry {
            timestamp: Utc::now().timestamp(),
            action: action.to_string(),
            user: user.to_string(),
            result: result.to_string(),
            details: None,
        };

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            let line = format!(
                "[{}] {} | {} | {} | {}\n",
                entry.timestamp,
                entry.user,
                entry.action,
                entry.result,
                entry.details.as_deref().unwrap_or("")
            );
            let _ = file.write_all(line.as_bytes());
        }
    }

    pub fn query(&self, _filter: AuditFilter) -> Vec<AuditEntry> {
        Vec::new()
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

pub struct AuditFilter {
    pub user: Option<String>,
    pub action: Option<String>,
    pub result: Option<String>,
    pub since: Option<i64>,
    pub until: Option<i64>,
    pub limit: usize,
}
