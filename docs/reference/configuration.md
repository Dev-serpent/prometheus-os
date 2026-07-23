# Configuration Reference

Prometheus OS uses TOML configuration files located in `/etc/prometheus/`.

## Compositor Configuration

**File:** `/etc/prometheus/compositor.conf`

```toml
[display]
max_fps = 240
vsync = true
adaptive_sync = true
background_color = "#0A0A0A"

[effects]
blur = true
blur_strength = 12
glow = true
shadow = true
shadow_strength = 0.5
animation_duration_ms = 200

[workspaces]
count = 9
layout = "dynamic"   # dynamic, master-stack, grid
switch_animation = "slide"

[input]
touchpad_enabled = true
touchpad_natural_scroll = true
touchpad_tap_to_click = true
mouse_acceleration = true
mouse_speed = 0.5
keyboard_repeat_rate = 30
keyboard_repeat_delay = 250

[multi_monitor]
enabled = true
vrr_supported = true
arrangement = "right"    # right, left, above, below, mirror
primary = "auto"
```

## AI Core Configuration

**File:** `/etc/prometheus/ai.conf`

See [AI Core Configuration](../ai-core/config.md) for complete reference.

## System Configuration

**File:** `/etc/prometheus/prometheus.conf`

```toml
[desktop]
preferred_environment = "gnome"      # gnome, native
autostart_ai = true
theme = "prometheus-dark"
language = "en-US"
first_run = false

[updates]
auto_check = true
auto_download = false
channel = "stable"                   # stable, beta, nightly

[privacy]
send_analytics = false
crash_reporting = true
machine_learning_local = true
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PROMETHEUS_CONFIG_DIR` | `/etc/prometheus` | Configuration directory |
| `PROMETHEUS_DATA_DIR` | `/var/lib/prometheus` | Data directory |
| `PROMETHEUS_LOG_DIR` | `/var/log/prometheus` | Log directory |
| `PROMETHEUS_AI_CONFIG` | `$PROMETHEUS_CONFIG_DIR/ai.conf` | AI Core config |
| `PROMETHEUS_COMPOSITOR_CONFIG` | `$PROMETHEUS_CONFIG_DIR/compositor.conf` | Compositor config |
| `PROMETHEUS_LOG_LEVEL` | `info` | Log level |
| `PROMETHEUS_MODEL_PATH` | `/usr/lib/prometheus/models` | AI model path |
| `RUST_LOG` | `info` | Rust log level (overrides PROMETHEUS_LOG_LEVEL) |

## File Locations

| Path | Purpose |
|------|---------|
| `/etc/prometheus/` | Configuration files |
| `/var/lib/prometheus/` | Runtime data and databases |
| `/var/log/prometheus/` | Log files |
| `/usr/lib/prometheus/` | Libraries and models |
| `/usr/share/prometheus/` | Shared data and defaults |
| `/usr/share/backgrounds/prometheus/` | Wallpapers |
| `/usr/share/themes/Prometheus-Dark/` | GNOME theme |
| `/usr/share/gnome-shell/extensions/prometheus-ai@prometheus-os.dev/` | AI extension |
| `/usr/share/plymouth/themes/prometheus/` | Boot theme |
