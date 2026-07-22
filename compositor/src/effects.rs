use crate::config::CompositorConfig;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct BlurManager {
    config: Arc<RwLock<CompositorConfig>>,
    enabled: bool,
    radius: u32,
    passes: u32,
}

impl BlurManager {
    pub fn new(config: Arc<RwLock<CompositorConfig>>) -> Self {
        let enabled = config.read().effects.enable_blur;
        let radius = config.read().effects.blur_size;
        Self {
            enabled,
            radius,
            passes: 3,
            config,
        }
    }

    pub fn apply(&self, _surface: &SurfaceId) {
        if !self.enabled {
            return;
        }
        self.render_blur();
    }

    fn render_blur(&self) {}
}

pub struct GlowManager {
    config: Arc<RwLock<CompositorConfig>>,
    enabled: bool,
    intensity: f32,
}

impl GlowManager {
    pub fn new(config: Arc<RwLock<CompositorConfig>>) -> Self {
        let enabled = config.read().effects.enable_glow;
        let intensity = config.read().effects.glow_intensity;
        Self {
            enabled,
            intensity,
            config,
        }
    }

    pub fn apply(&self, _surface: &SurfaceId) {
        if !self.enabled {
            return;
        }
        self.render_glow();
    }

    fn render_glow(&self) {}
}

pub struct ShadowManager {
    config: Arc<RwLock<CompositorConfig>>,
    enabled: bool,
    size: u32,
}

impl ShadowManager {
    pub fn new(config: Arc<RwLock<CompositorConfig>>) -> Self {
        let enabled = config.read().effects.enable_shadows;
        let size = config.read().effects.shadow_size;
        Self {
            enabled,
            size,
            config,
        }
    }

    pub fn apply(&self, _surface: &SurfaceId) {
        if !self.enabled {
            return;
        }
        self.render_shadow();
    }

    fn render_shadow(&self) {}
}

pub struct SurfaceId(u64);
