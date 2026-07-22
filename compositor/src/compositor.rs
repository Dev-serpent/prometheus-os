use crate::config::CompositorConfig;
use crate::effects::{BlurManager, GlowManager, ShadowManager};
use crate::input::InputManager;
use crate::layout::LayoutManager;
use crate::render::Renderer;
use crate::shell::ShellIntegration;
use crate::workspace::WorkspaceManager;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct PrometheusCompositor {
    config: Arc<RwLock<CompositorConfig>>,
    renderer: Renderer,
    input: InputManager,
    layout: LayoutManager,
    workspaces: WorkspaceManager,
    effects: EffectsManager,
    shell: ShellIntegration,
    running: bool,
}

struct EffectsManager {
    blur: BlurManager,
    glow: GlowManager,
    shadow: ShadowManager,
}

impl PrometheusCompositor {
    pub fn new() -> anyhow::Result<Self> {
        let config = Arc::new(RwLock::new(CompositorConfig::load(None)));

        Ok(Self {
            renderer: Renderer::new()?,
            input: InputManager::new(),
            layout: LayoutManager::new(),
            workspaces: WorkspaceManager::new(9),
            effects: EffectsManager {
                blur: BlurManager::new(config.clone()),
                glow: GlowManager::new(config.clone()),
                shadow: ShadowManager::new(config.clone()),
            },
            shell: ShellIntegration::new(config.clone()),
            config,
            running: false,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.running = true;
        tracing::info!("Prometheus Compositor started");

        while self.running {
            self.tick();
        }

        Ok(())
    }

    fn tick(&mut self) {
        self.input.poll();
        self.layout.update();
        self.workspaces.update();
        self.renderer.frame(&self.config, &self.layout, &self.workspaces);
    }

    pub fn shutdown(&mut self) {
        self.running = false;
        tracing::info!("Prometheus Compositor shutting down");
    }
}
