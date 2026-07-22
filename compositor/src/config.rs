use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositorConfig {
    pub display: DisplayConfig,
    pub animations: AnimationConfig,
    pub effects: EffectsConfig,
    pub input: InputConfig,
    pub workspaces: WorkspaceConfig,
    pub layout: LayoutConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub max_fps: u32,
    pub enable_vrr: bool,
    pub enable_hdr: bool,
    pub scale: f64,
    pub background_color: [u8; 4],
    pub accent_color: [u8; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub enable_animations: bool,
    pub animation_speed: f32,
    pub physics_based: bool,
    pub spring_stiffness: f32,
    pub spring_damping: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsConfig {
    pub enable_blur: bool,
    pub blur_size: u32,
    pub enable_shadows: bool,
    pub shadow_size: u32,
    pub corner_radius: f32,
    pub enable_glow: bool,
    pub glow_intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub touchpad_natural_scroll: bool,
    pub touchpad_tap_to_click: bool,
    pub gesture_enabled: bool,
    pub pointer_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub count: u32,
    pub enable_animation: bool,
    pub wrap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub default_tiling: bool,
    pub gap_size: u32,
    pub border_size: u32,
    pub master_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub direct_scanout: bool,
    pub tear_free: bool,
    pub allow_tearing: bool,
    pub idle_inhibit: bool,
    pub power_save: bool,
}

impl Default for CompositorConfig {
    fn default() -> Self {
        Self {
            display: DisplayConfig {
                max_fps: 240,
                enable_vrr: true,
                enable_hdr: false,
                scale: 1.0,
                background_color: [10, 10, 10, 255],
                accent_color: [0, 120, 255, 255],
            },
            animations: AnimationConfig {
                enable_animations: true,
                animation_speed: 1.0,
                physics_based: true,
                spring_stiffness: 300.0,
                spring_damping: 25.0,
            },
            effects: EffectsConfig {
                enable_blur: true,
                blur_size: 8,
                enable_shadows: true,
                shadow_size: 12,
                corner_radius: 12.0,
                enable_glow: true,
                glow_intensity: 0.3,
            },
            input: InputConfig {
                touchpad_natural_scroll: true,
                touchpad_tap_to_click: true,
                gesture_enabled: true,
                pointer_speed: 0.5,
            },
            workspaces: WorkspaceConfig {
                count: 9,
                enable_animation: true,
                wrap: true,
            },
            layout: LayoutConfig {
                default_tiling: true,
                gap_size: 8,
                border_size: 2,
                master_factor: 0.6,
            },
            performance: PerformanceConfig {
                direct_scanout: true,
                tear_free: true,
                allow_tearing: false,
                idle_inhibit: true,
                power_save: true,
            },
        }
    }
}

impl CompositorConfig {
    pub fn load(path: Option<PathBuf>) -> Self {
        let path = path.unwrap_or_else(|| PathBuf::from("/etc/prometheus/compositor.conf"));
        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}
