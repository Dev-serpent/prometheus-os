use std::collections::HashMap;
use parking_lot::RwLock;

pub struct PermissionManager {
    permissions: RwLock<HashMap<String, PermissionEntry>>,
    defaults: DefaultPermissions,
}

struct PermissionEntry {
    allowed: bool,
    permanent: bool,
    expires_at: Option<i64>,
    approval_required: bool,
}

struct DefaultPermissions {
    read: bool,
    write: bool,
    execute: bool,
    network: bool,
    audio: bool,
    video: bool,
    location: bool,
    notifications: bool,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            permissions: RwLock::new(HashMap::new()),
            defaults: DefaultPermissions {
                read: true,
                write: false,
                execute: false,
                network: true,
                audio: true,
                video: false,
                location: false,
                notifications: true,
            },
        }
    }

    pub fn initialize(&self) {
        tracing::info!("Permission manager initialized");
    }

    pub fn check(&self, action: &str, _target: &str) -> bool {
        let permissions = self.permissions.read();

        if let Some(entry) = permissions.get(action) {
            return entry.allowed;
        }

        // Check defaults
        match action {
            "read" => self.defaults.read,
            "write" => self.defaults.write,
            "execute" => self.defaults.execute,
            "network" => self.defaults.network,
            "audio" => self.defaults.audio,
            "video" => self.defaults.video,
            "location" => self.defaults.location,
            "notifications" => self.defaults.notifications,
            _ => false,
        }
    }

    pub fn grant(&self, action: &str, permanent: bool) {
        let mut permissions = self.permissions.write();
        permissions.insert(action.to_string(), PermissionEntry {
            allowed: true,
            permanent,
            expires_at: if permanent { None } else { Some(chrono::Utc::now().timestamp() + 3600) },
            approval_required: false,
        });
    }

    pub fn revoke(&self, action: &str) {
        let mut permissions = self.permissions.write();
        permissions.remove(action);
    }
}
