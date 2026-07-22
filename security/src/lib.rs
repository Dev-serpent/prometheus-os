pub mod sandbox;
pub mod permissions;
pub mod audit;
pub mod encryption;

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct SecurityManager {
    sandbox: sandbox::SandboxManager,
    permissions: permissions::PermissionManager,
    audit: audit::AuditLogger,
    encryption: encryption::MemoryEncryption,
    policies: Arc<RwLock<SecurityPolicies>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityPolicies {
    pub secure_boot_required: bool,
    pub sandbox_all_apps: bool,
    pub audit_all_actions: bool,
    pub require_ai_approval: bool,
    pub confirm_destructive: bool,
    pub lockdown_mode: LockdownLevel,
    pub allowed_capabilities: Vec<String>,
    pub restricted_paths: Vec<String>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum LockdownLevel {
    None,
    Integrity,
    Confidentiality,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            sandbox: sandbox::SandboxManager::new(),
            permissions: permissions::PermissionManager::new(),
            audit: audit::AuditLogger::new(),
            encryption: encryption::MemoryEncryption::new(),
            policies: Arc::new(RwLock::new(SecurityPolicies {
                secure_boot_required: true,
                sandbox_all_apps: true,
                audit_all_actions: true,
                require_ai_approval: true,
                confirm_destructive: true,
                lockdown_mode: LockdownLevel::Integrity,
                allowed_capabilities: vec![
                    "read".into(), "write".into(), "execute".into(),
                    "network".into(), "audio".into(), "video".into(),
                ],
                restricted_paths: vec![
                    "/etc".into(), "/usr".into(), "/boot".into(),
                    "/var/lib/prometheus".into(),
                ],
            })),
        }
    }

    pub fn initialize(&self) -> anyhow::Result<()> {
        self.sandbox.initialize()?;
        self.permissions.initialize();
        self.audit.initialize()?;
        self.encryption.initialize();

        if self.policies.read().secure_boot_required {
            self.verify_secure_boot()?;
        }

        Ok(())
    }

    fn verify_secure_boot(&self) -> anyhow::Result<()> {
        let status = std::process::Command::new("sbctl")
            .arg("status")
            .output()?;

        if !status.status.success() {
            tracing::warn!("Secure Boot is not enabled");
        }

        Ok(())
    }

    pub fn check_permission(&self, action: &str, target: &str) -> bool {
        self.permissions.check(action, target)
    }

    pub fn log_action(&self, action: &str, user: &str, result: &str) {
        self.audit.log(action, user, result);
    }

    pub fn sandbox_process(&self, command: &str) -> anyhow::Result<()> {
        self.sandbox.run_sandboxed(command)
    }
}
