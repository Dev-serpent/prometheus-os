<p align="center">
  <img src="https://raw.githubusercontent.com/sudorootvector/prometheus-os/main/resources/brand/prometheus-logo-dark.svg" width="280" alt="Prometheus OS">
</p>

<h1 align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://readme-typing-svg.demolab.com?font=Inter&weight=700&size=32&duration=4000&pause=500&color=0078FF&center=true&vCenter=true&repeat=true&width=500&lines=AI-Native+Operating+System;Built+for+2040;The+OS+is+the+AI">
    <img src="https://readme-typing-svg.demolab.com?font=Inter&weight=700&size=32&duration=4000&pause=500&color=0078FF&center=true&vCenter=true&repeat=true&width=500&lines=AI-Native+Operating+System;Built+for+2040;The+OS+is+the+AI">
  </picture>
</h1>

<p align="center">
  <a href="https://github.com/sudorootvector/prometheus-os"><img src="https://img.shields.io/badge/status-active--development-0A0A0A?style=flat&logo=github" alt="Status"></a>
  <a href="https://github.com/sudorootvector/prometheus-os/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-GPLv3-0A0A0A?style=flat" alt="License"></a>
  <a href="#"><img src="https://img.shields.io/badge/kernel-linux--zen-0A0A0A?style=flat&logo=linux" alt="Kernel"></a>
  <a href="#"><img src="https://img.shields.io/badge/display-wayland-0A0A0A?style=flat" alt="Display"></a>
  <a href="#"><img src="https://img.shields.io/badge/language-rust-0A0A0A?style=flat&logo=rust" alt="Language"></a>
  <a href="#"><img src="https://img.shields.io/badge/ram_idle-%3C900MB-0A0A0A?style=flat" alt="RAM"></a>
</p>

---

Prometheus OS is not another Linux distribution. Applications become tools the AI uses. You communicate with Prometheus instead of manually managing software.

---

## Quick Start

```bash
git clone https://github.com/sudorootvector/prometheus-os.git
cd prometheus-os
make all && make iso
```

---

## Why Prometheus?

**Typical OS** → You open apps, manage windows, search for files, run commands.

**Prometheus** → You talk to the AI. It opens apps, manages windows, finds files, runs commands. It learns how you work and automates repetitive tasks. The desktop becomes an intelligent workspace.

---

## Architecture

```
You → AI Core → Reasoning → Memory → Action → Desktop
         ↑_________________________________↓
              Continuous learning loop
```

| Layer | What it does |
|-------|-------------|
| **AI Core** | Reasoning, memory, vision, voice, automation |
| **Compositor** | wlroots-based, 240 FPS, GPU-accelerated |
| **Desktop** | Panel, launcher, notifications, workspaces |
| **Security** | Sandbox, permissions, audit, encryption |
| **Apps** | Terminal, files, settings, dashboard, 6 more |
| **Foundation** | Arch Linux, linux-zen, systemd, Wayland, Btrfs |

---

## AI Capabilities

- **Desktop understanding** — knows what apps and windows you have open
- **Screen vision** — reads text, detects UI elements, understands context
- **Voice control** — wake-word activated speech-to-text and response
- **Memory graph** — persistent knowledge store with semantic search
- **Workflow learning** — observes patterns, suggests automations
- **Multi-agent reasoning** — decomposes complex tasks across agents

---

## Desktop Environments

**Option 1: Prometheus Native**
Custom wlroots compositor with GPU rendering, physics animations, dynamic tiling, blur, glow, 9 workspaces.

**Option 2: GNOME** (fully integrated)
Dark glassmorphism theme, live AI panel indicator, real-time system monitor in the top bar, dynamic window tiling, custom GDM login.

---

## Applications

Terminal · Files · Settings · Dashboard · Package Manager · System Monitor · Snapshot Manager · Plugin Manager · Developer Hub

All native Rust applications sharing one design language.

---

## Performance Targets

- Idle RAM: **< 900 MB**
- Cold boot: **< 5 seconds**
- Compositor: **240 FPS**
- AI response: **< 100 ms**
- Wake from suspend: **< 500 ms**

---

## Security

Secure Boot · Sandboxed apps · Permission-based AI actions · Audit logging · Memory encryption · User-confirm destructive ops

---

## SDK

```rust
// Rust
let sdk = PrometheusSDK::new();
sdk.ai().query("What's on my screen?");
```

```python
# Python
from prometheus_sdk import PrometheusSDK
PrometheusSDK().ai.query("Analyze my system")
```

```javascript
// JavaScript
const { PrometheusSDK } = require('prometheus-sdk');
await PrometheusSDK().ai.query('Summarize this page');
```

---

## Robotics

ROS2 · CAN Bus · Serial · GPIO · LiDAR · Depth cameras · Robotic arms · Drones

The AI Core interfaces directly with hardware for autonomous robot control.

---

## Build from Source

```bash
make all           # Build core (Rust)
make gnome         # Build GNOME integration
make install       # Install to /usr
make iso           # Bootable ISO
```

**Requires:** Arch Linux, base-devel, rust, cargo

---

## Project

```
132 files · 9,796 lines · 15 Rust crates · GPLv3
```

<p align="center">
  <sub>Built with 🔥 by the Prometheus OS Team</sub>
</p>
