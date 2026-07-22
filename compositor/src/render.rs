use crate::config::CompositorConfig;
use std::fs;
use std::sync::Arc;
use std::collections::VecDeque;
use parking_lot::RwLock;

pub struct Renderer {
    gpu: GpuState,
    frame_stats: FrameStats,
}

#[derive(Clone)]
pub struct GpuState {
    pub device: String,
    pub vram_used: u64,
    pub vram_total: u64,
    pub temp_celsius: f32,
}

pub struct FrameStats {
    pub fps: u32,
    pub frame_time_ns: u64,
    pub frames_rendered: u64,
    pub vsync_enabled: bool,
    pub last_present_time: std::time::Instant,
    pub frame_times: VecDeque<u64>,
    pub dropped_frames: u64,
    pub max_frame_time_ns: u64,
    pub min_frame_time_ns: u64,
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
    pub opacity: f32,
    pub scale: f32,
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
            opacity: 1.0,
            scale: 1.0,
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

    pub fn set_position(&mut self, x: i32, y: i32) { self.x = x; self.y = y; }
    pub fn set_size(&mut self, w: i32, h: i32) { self.width = w; self.height = h; }
    pub fn set_focused(&mut self, focused: bool) { self.focused = focused; }
    pub fn set_title(&mut self, title: String) { self.title = title; }
    pub fn set_app_id(&mut self, app_id: String) { self.app_id = app_id; }
}

impl Renderer {
    pub fn new() -> anyhow::Result<Self> {
        let gpu = Self::detect_gpu();
        Ok(Self {
            gpu,
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
        })
    }

    fn detect_gpu() -> GpuState {
        let mut device = String::new();
        let mut temp = 0.0;

        if let Ok(_pci) = fs::read_to_string("/sys/bus/pci/devices") {
            for entry in fs::read_dir("/sys/class/drm").unwrap_or_else(|_| fs::read_dir("/dev/dri").unwrap()) {
                if let Ok(entry) = entry {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("card") && !name.contains('-') {
                        let path = format!("/sys/class/drm/{}/device/uevent", name);
                        if let Ok(uevent) = fs::read_to_string(&path) {
                            for line in uevent.lines() {
                                if line.starts_with("DRIVER=") {
                                    device = line.trim_start_matches("DRIVER=").to_string();
                                }
                            }
                        }
                        let temp_path = format!("/sys/class/drm/{}/device/hwmon/hwmon0/temp1_input", name);
                        if let Ok(t) = fs::read_to_string(&temp_path) {
                            temp = t.trim().parse::<f32>().unwrap_or(0.0) / 1000.0;
                        }
                        break;
                    }
                }
            }
        }

        GpuState {
            device,
            vram_used: 0,
            vram_total: 0,
            temp_celsius: temp,
        }
    }

    pub fn frame(
        &mut self,
        _config: &Arc<RwLock<CompositorConfig>>,
    ) {
        let frame_start = std::time::Instant::now();

        self.begin_frame();
        self.commit_frame();

        let elapsed = frame_start.elapsed().as_nanos() as u64;
        self.frame_stats.frame_time_ns = elapsed;
        self.frame_stats.frames_rendered += 1;

        self.frame_stats.frame_times.push_back(elapsed);
        if self.frame_stats.frame_times.len() > 60 {
            self.frame_stats.frame_times.pop_front();
        }

        if self.frame_stats.frame_times.len() >= 2 {
            let total: u64 = self.frame_stats.frame_times.iter().sum();
            let avg = total / self.frame_stats.frame_times.len() as u64;
            self.frame_stats.fps = if avg > 0 { 1_000_000_000 / avg as u64 } else { 0 } as u32;
        }

        self.frame_stats.max_frame_time_ns = self.frame_stats.max_frame_time_ns.max(elapsed);
        self.frame_stats.min_frame_time_ns = self.frame_stats.min_frame_time_ns.min(elapsed);

        // V-Sync timing for 240 FPS target
        if elapsed < 4_166_667 {
            spin_sleep(4_166_667 - elapsed);
        }

        if elapsed > 8_333_334 {
            self.frame_stats.dropped_frames += 1;
        }
    }

    fn begin_frame(&self) {}
    fn commit_frame(&self) {}

    pub fn draw_window_background(&self, _window: &WindowHandle, _blur: bool, _shadow: bool) {}
    pub fn draw_window_decorations(&self, _window: &WindowHandle) {}
    pub fn draw_panel(&self) {}

    pub fn gpu_info(&self) -> &GpuState { &self.gpu }
    pub fn fps(&self) -> u32 { self.frame_stats.fps }
    pub fn frame_stats(&self) -> &FrameStats { &self.frame_stats }
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
