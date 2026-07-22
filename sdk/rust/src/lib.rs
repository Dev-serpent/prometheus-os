pub mod ai;
pub mod app;
pub mod desktop;
pub mod system;


pub struct PrometheusSDK {
    version: String,
    ai: ai::AIClient,
    app: app::AppFramework,
    desktop: desktop::DesktopAPI,
    system: system::SystemAPI,
}

impl PrometheusSDK {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            ai: ai::AIClient::new(),
            app: app::AppFramework::new(),
            desktop: desktop::DesktopAPI::new(),
            system: system::SystemAPI::new(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn ai(&self) -> &ai::AIClient {
        &self.ai
    }

    pub fn app(&self) -> &app::AppFramework {
        &self.app
    }

    pub fn desktop(&self) -> &desktop::DesktopAPI {
        &self.desktop
    }

    pub fn system(&self) -> &system::SystemAPI {
        &self.system
    }
}
