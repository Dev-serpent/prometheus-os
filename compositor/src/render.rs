use crate::config::CompositorConfig;
use crate::layout::LayoutManager;
use crate::workspace::WorkspaceManager;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct Renderer {
    gpu: GpuState,
    pipelines: Vec<RenderPipeline>,
    frame_stats: FrameStats,
}

struct GpuState {
    device: String,
    vram_used: u64,
    vram_total: u64,
    temp_celsius: f32,
}

struct RenderPipeline {
    blur_pipeline: Option<PipelineHandle>,
    glow_pipeline: Option<PipelineHandle>,
    shadow_pipeline: Option<PipelineHandle>,
    solid_pipeline: PipelineHandle,
}

struct PipelineHandle(u64);

struct FrameStats {
    fps: u32,
    frame_time_ns: u64,
    frames_rendered: u64,
    vsync_enabled: bool,
}

impl Renderer {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            gpu: GpuState {
                device: String::new(),
                vram_used: 0,
                vram_total: 0,
                temp_celsius: 0.0,
            },
            pipelines: Vec::new(),
            frame_stats: FrameStats {
                fps: 0,
                frame_time_ns: 0,
                frames_rendered: 0,
                vsync_enabled: true,
            },
        })
    }

    pub fn frame(
        &mut self,
        config: &Arc<RwLock<CompositorConfig>>,
        layout: &LayoutManager,
        workspaces: &WorkspaceManager,
    ) {
        let cfg = config.read();
        self.begin_frame();

        for window in layout.visible_windows() {
            self.render_window_background(&cfg, window);
            self.render_window_content(window);
            self.render_window_decorations(&cfg, window);
        }

        self.render_panel(&cfg);
        self.commit_frame(&cfg);
    }

    fn begin_frame(&mut self) {
        let start = std::time::Instant::now();
        self.frame_stats.frames_rendered += 1;
        self.frame_stats.frame_time_ns = start.elapsed().as_nanos() as u64;
    }

    fn render_window_background(
        &self,
        cfg: &CompositorConfig,
        window: &WindowHandle,
    ) {
        if cfg.effects.enable_blur {
            self.apply_blur(window, cfg.effects.blur_size);
        }
        if cfg.effects.enable_shadows {
            self.apply_shadow(window, cfg.effects.shadow_size);
        }
        self.draw_rounded_rect(
            window.x(),
            window.y(),
            window.width(),
            window.height(),
            cfg.effects.corner_radius,
        );
    }

    fn render_window_decorations(
        &self,
        cfg: &CompositorConfig,
        window: &WindowHandle,
    ) {
        if window.is_focused() {
            self.draw_border(
                window.x() - 1,
                window.y() - 1,
                window.width() + 2,
                window.height() + 2,
                cfg.effects.corner_radius + 1.0,
                &cfg.display.accent_color,
            );
            if cfg.effects.enable_glow {
                self.apply_glow(window, cfg.effects.glow_intensity);
            }
        }
    }

    fn render_panel(&self, cfg: &CompositorConfig) {
        self.draw_rounded_rect(0, 0, 1920, 48, 0);
    }

    fn commit_frame(&mut self, cfg: &CompositorConfig) {
        let target_fps = cfg.display.max_fps;
        let target_frame_ns = 1_000_000_000 / target_fps as u64;
        let elapsed = self.frame_stats.frame_time_ns;

        if elapsed < target_frame_ns {
            std::thread::sleep(std::time::Duration::from_nanos(
                target_frame_ns - elapsed,
            ));
        }

        self.frame_stats.fps = if elapsed > 0 {
            1_000_000_000 / elapsed
        } else {
            target_fps
        };
    }

    fn apply_blur(&self, _window: &WindowHandle, _size: u32) {}
    fn apply_shadow(&self, _window: &WindowHandle, _size: u32) {}
    fn apply_glow(&self, _window: &WindowHandle, _intensity: f32) {}
    fn draw_rounded_rect(&self, _x: i32, _y: i32, _w: i32, _h: i32, _r: f32) {}
    fn draw_border(&self, _x: i32, _y: i32, _w: i32, _h: i32, _r: f32, _color: &[u8; 4]) {}

    pub fn gpu_info(&self) -> GpuState {
        self.gpu.clone()
    }

    pub fn fps(&self) -> u32 {
        self.frame_stats.fps
    }
}

#[derive(Clone)]
pub struct WindowHandle {
    id: u64,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    focused: bool,
    title: String,
    app_id: String,
}

impl WindowHandle {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            x: 0,
            y: 0,
            width: 1024,
            height: 768,
            focused: false,
            title: String::new(),
            app_id: String::new(),
        }
    }

    pub fn id(&self) -> u64 { self.id }
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
    pub fn is_focused(&self) -> bool { self.focused }
    pub fn title(&self) -> &str { &self.title }
    pub fn app_id(&self) -> &str { &self.app_id }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
}
