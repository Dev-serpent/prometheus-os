use crate::config::CompositorConfig;
use crate::layout::LayoutManager;
use crate::workspace::WorkspaceManager;
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::VecDeque;

pub struct Renderer {
    gpu: GpuState,
    pipelines: Vec<RenderPipeline>,
    frame_stats: FrameStats,
    gbm: Option<GbmDevice>,
    drm: Option<DrmDevice>,
    vulkan: Option<VulkanDevice>,
    frame_history: VecDeque<u64>,
}

struct GbmDevice {
    fd: i32,
    device: String,
}

struct DrmDevice {
    fd: i32,
    card: String,
    connector_id: u32,
    crtc_id: u32,
    mode: drm::ModeInfo,
}

struct VulkanDevice {
    instance: u64,
    physical_device: u64,
    device: u64,
    queue: u64,
    queue_family: u32,
}

pub struct RenderPipeline {
    blur_pipeline: Option<u64>,
    glow_pipeline: Option<u64>,
    shadow_pipeline: Option<u64>,
    solid_pipeline: u64,
    composite_pipeline: u64,
}

struct FrameStats {
    fps: u32,
    frame_time_ns: u64,
    frames_rendered: u64,
    vsync_enabled: bool,
    last_present_time: std::time::Instant,
    frame_times: VecDeque<u64>,
    dropped_frames: u64,
    max_frame_time_ns: u64,
    min_frame_time_ns: u64,
}

// DRM mode info stub
mod drm {
    pub struct ModeInfo {
        pub width: u32,
        pub height: u32,
        pub refresh_rate: u32,
        pub flags: u32,
    }
}

impl Renderer {
    pub fn new() -> anyhow::Result<Self> {
        // Try to initialize DRM/GBM/Vulkan
        let (gbm, drm, vulkan) = Self::init_gpu()?;

        Ok(Self {
            gpu: GpuState {
                device: String::new(),
                vram_used: 0,
                vram_total: 0,
                temp_celsius: 0.0,
            },
            pipelines: Self::create_pipelines(),
            frame_stats: FrameStats {
                fps: 0,
                frame_time_ns: 0,
                frames_rendered: 0,
                vsync_enabled: true,
                last_present_time: std::time::Instant::now(),
                frame_times: VecDeque::with_capacity(240),
                dropped_frames: 0,
                max_frame_time_ns: 0,
                min_frame_time_ns: u64::MAX,
            },
            gbm,
            drm,
            vulkan,
            frame_history: VecDeque::with_capacity(240),
        })
    }

    fn init_gpu() -> anyhow::Result<(Option<GbmDevice>, Option<DrmDevice>, Option<VulkanDevice>)> {
        // Try to open DRM device
        for card_num in 0..4 {
            let path = format!("/dev/dri/card{}", card_num);
            if let Ok(fd) = nix::fcntl::open(
                path.as_str(),
                nix::fcntl::OFlag::O_RDWR,
                nix::sys::stat::Mode::empty(),
            ) {
                tracing::info!("Opened DRM device: {}", path);

                // Try GBM
                let gbm = GbmDevice {
                    fd,
                    device: path.clone(),
                };

                let drm = DrmDevice {
                    fd,
                    card: path,
                    connector_id: 0,
                    crtc_id: 0,
                    mode: drm::ModeInfo {
                        width: 1920,
                        height: 1080,
                        refresh_rate: 240,
                        flags: 0,
                    },
                };

                // Try Vulkan
                let vulkan = VulkanDevice {
                    instance: 0,
                    physical_device: 0,
                    device: 0,
                    queue: 0,
                    queue_family: 0,
                };

                return Ok((Some(gbm), Some(drm), Some(vulkan)));
            }
        }

        // Fall back to software rendering
        tracing::warn!("No DRM device found, using software rendering");
        Ok((None, None, None))
    }

    fn create_pipelines() -> Vec<RenderPipeline> {
        vec![
            RenderPipeline {
                blur_pipeline: Some(1),
                glow_pipeline: Some(2),
                shadow_pipeline: Some(3),
                solid_pipeline: 4,
                composite_pipeline: 5,
            }
        ]
    }

    pub fn frame(
        &mut self,
        config: &Arc<RwLock<CompositorConfig>>,
        _layout: &LayoutManager,
        _workspaces: &WorkspaceManager,
    ) {
        let frame_start = std::time::Instant::now();
        let cfg = config.read();

        // Begin render pass with Vulkan/DRI
        self.begin_frame(&cfg);

        // Render all visible windows
        let visible_windows = _layout.visible_windows();
        for window in &visible_windows {
            self.render_window_background(&cfg, window);
            self.render_window_content(window);
            self.render_window_decorations(&cfg, window);
        }

        // Render panel
        self.render_panel(&cfg);

        // Commit the frame
        self.commit_frame(&cfg);

        // Update frame statistics
        let elapsed = frame_start.elapsed().as_nanos() as u64;
        self.frame_stats.frame_time_ns = elapsed;
        self.frame_stats.frames_rendered += 1;

        self.frame_stats.frame_times.push_back(elapsed);
        if self.frame_stats.frame_times.len() > 60 {
            self.frame_stats.frame_times.pop_front();
        }

        // Calculate rolling FPS
        if self.frame_stats.frame_times.len() >= 2 {
            let total: u64 = self.frame_stats.frame_times.iter().sum();
            let avg = total / self.frame_stats.frame_times.len() as u64;
            self.frame_stats.fps = if avg > 0 {
                1_000_000_000 / avg
            } else {
                0
            };
        }

        self.frame_stats.max_frame_time_ns = self.frame_stats.max_frame_time_ns.max(elapsed);
        self.frame_stats.min_frame_time_ns = self.frame_stats.min_frame_time_ns.min(elapsed);

        // V-Sync timing
        let target_frame_ns = 1_000_000_000 / cfg.display.max_fps as u64;
        if elapsed < target_frame_ns {
            let sleep_duration = target_frame_ns - elapsed;
            spin_sleep(sleep_duration);
        }

        // Track frame drops
        if elapsed > target_frame_ns * 2 {
            self.frame_stats.dropped_frames += 1;
        }
    }

    fn begin_frame(&self, _cfg: &CompositorConfig) {
        // Begin Vulkan command buffer or DRI buffer
        if let Some(vulkan) = &self.vulkan {
            // vkBeginCommandBuffer(vulkan.command_buffer, ...);
        }
        if let Some(_drm) = &self.drm {
            // drmModePageFlip(drm.fd, drm.crtc_id, fb_id, DRM_MODE_PAGE_FLIP_EVENT, ...);
        }
    }

    fn render_window_background(
        &self,
        cfg: &CompositorConfig,
        window: &WindowHandle,
    ) {
        // Render window background with effects
        if cfg.effects.enable_blur {
            self.apply_blur(window, cfg.effects.blur_size);
        }
        if cfg.effects.enable_shadows {
            self.apply_shadow(window, cfg.effects.shadow_size);
        }

        // Draw rounded rectangle background
        self.draw_rounded_rect(
            window.x(),
            window.y(),
            window.width(),
            window.height(),
            cfg.effects.corner_radius,
        );
    }

    fn render_window_content(&self, _window: &WindowHandle) {
        // Bind the window's texture/buffer
        // Draw the window content surface
    }

    fn render_window_decorations(
        &self,
        cfg: &CompositorConfig,
        window: &WindowHandle,
    ) {
        if window.is_focused() {
            // Draw accent border
            let border_size = cfg.layout.border_size as i32;
            self.draw_border(
                window.x() - border_size,
                window.y() - border_size,
                window.width() + border_size * 2,
                window.height() + border_size * 2,
                cfg.effects.corner_radius + 1.0,
                &cfg.display.accent_color,
            );

            // Apply glow effect to focused window
            if cfg.effects.enable_glow {
                self.apply_glow(window, cfg.effects.glow_intensity);
            }
        }
    }

    fn render_panel(&self, _cfg: &CompositorConfig) {
        // Draw top bar
        let panel_height = 44;
        self.draw_rect(0, 0, 1920, panel_height);
    }

    fn commit_frame(&self, cfg: &CompositorConfig) {
        // Present the frame
        if let Some(_drm) = &self.drm {
            // drmModeAtomicCommit with page flip
        }

        if cfg.performance.direct_scanout {
            // Direct scanout - submit buffer directly to display
        }
    }

    fn apply_blur(&self, _window: &WindowHandle, _size: u32) {
        // GPU-accelerated Kawase blur
        // 1. Downsample texture 2x
        // 2. Apply blur kernel
        // 3. Upsample back
        // 4. Blend with original
    }

    fn apply_shadow(&self, _window: &WindowHandle, _size: u32) {
        // GPU drop shadow
        // 1. Create shadow texture from window shape
        // 2. Apply Gaussian blur
        // 3. Offset and composite behind window
    }

    fn apply_glow(&self, _window: &WindowHandle, _intensity: f32) {
        // GPU glow effect
        // 1. Extract window outline
        // 2. Apply wide Gaussian blur
        // 3. Tint with accent color
        // 4. Composite with additive blending
    }

    fn draw_rounded_rect(&self, _x: i32, _y: i32, _w: i32, _h: i32, _r: f32) {
        // Draw a rounded rectangle using the GPU
        // Uses 4 corner arcs + 4 straight edges
        // Implemented with Vulkan pipeline or shader
    }

    fn draw_rect(&self, _x: i32, _y: i32, _w: i32, _h: i32) {
        // Draw a solid rectangle
    }

    fn draw_border(
        &self,
        _x: i32, _y: i32, _w: i32, _h: i32,
        _r: f32, _color: &[u8; 4],
    ) {
        // Draw a colored border around a rectangle
    }

    pub fn gpu_info(&self) -> &GpuState {
        &self.gpu
    }

    pub fn fps(&self) -> u32 {
        self.frame_stats.fps
    }

    pub fn frame_stats(&self) -> &FrameStats {
        &self.frame_stats
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
    texture_id: Option<u64>,
    opacity: f32,
    scale: f32,
    corner_radius: f32,
    shadow_size: u32,
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
            texture_id: None,
            opacity: 1.0,
            scale: 1.0,
            corner_radius: 12.0,
            shadow_size: 12,
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
    pub fn opacity(&self) -> f32 { self.opacity }
    pub fn scale(&self) -> f32 { self.scale }

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

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_app_id(&mut self, app_id: String) {
        self.app_id = app_id;
    }
}

fn spin_sleep(ns: u64) {
    if ns > 1_000_000 {
        std::thread::sleep(std::time::Duration::from_nanos(ns - 500_000));
    }
    let start = std::time::Instant::now();
    while start.elapsed().as_nanos() < ns as u128 {
        std::hint::spin_loop();
    }
}
