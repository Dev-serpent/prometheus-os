<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/sudorootvector/prometheus-os/main/resources/brand/prometheus-logo-dark.svg">
    <img src="https://raw.githubusercontent.com/sudorootvector/prometheus-os/main/resources/brand/prometheus-logo-light.svg" width="320" alt="Prometheus OS">
  </picture>
</p>

<p align="center">
  <b>An AI-Native Operating System</b><br>
  <sub>Built on Arch Linux · Designed for 2040</sub>
</p>

<p align="center">
  <a href="#-architecture">
    <img src="https://img.shields.io/badge/ARCHITECTURE-0A0A0A?style=for-the-badge&logo=linux&logoColor=0078FF" alt="Architecture">
  </a>
  <a href="#-quick-start">
    <img src="https://img.shields.io/badge/QUICK_START-0A0A0A?style=for-the-badge&logo=gnubash&logoColor=00C853" alt="Quick Start">
  </a>
  <a href="#-core-components">
    <img src="https://img.shields.io/badge/COMPONENTS-0A0A0A?style=for-the-badge&logo=rust&logoColor=FFD600" alt="Components">
  </a>
  <a href="#-ai-core">
    <img src="https://img.shields.io/badge/AI_CORE-0A0A0A?style=for-the-badge&logo=tensorflow&logoColor=0078FF" alt="AI Core">
  </a>
  <a href="#-gnome-desktop">
    <img src="https://img.shields.io/badge/GNOME_DE-0A0A0A?style=for-the-badge&logo=gnome&logoColor=FFFFFF" alt="GNOME">
  </a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/STATUS-ACTIVE_DEVELOPMENT-00C853?style=flat-square&labelColor=0A0A0A" alt="Status">
  <img src="https://img.shields.io/badge/LICENSE-GPLv3-0078FF?style=flat-square&labelColor=0A0A0A" alt="License">
  <img src="https://img.shields.io/badge/KERNEL-LINUX_ZEN-FFD600?style=flat-square&labelColor=0A0A0A" alt="Kernel">
  <img src="https://img.shields.io/badge/DISPLAY-WAYLAND_ONLY-FFFFFF?style=flat-square&labelColor=0A0A0A" alt="Display">
  <img src="https://img.shields.io/badge/LANGUAGE-RUST-FF1744?style=flat-square&labelColor=0A0A0A" alt="Language">
  <img src="https://img.shields.io/badge/RAM_IDLE-%3C900MB-00C853?style=flat-square&labelColor=0A0A0A" alt="RAM">
</p>

<br>

---

```ascii
╔══════════════════════════════════════════════════════════════════╗
║                                                                  ║
║     ██████  ██████  ██████  ███    ███ ███████ ████████ ██   ██ ║
║     ██   ██ ██   ██ ██   ██ ████  ████ ██         ██    ██   ██ ║
║     ██████  ██████  ██   ██ ██ ████ ██ █████      ██    ███████ ║
║     ██      ██   ██ ██   ██ ██  ██  ██ ██         ██    ██   ██ ║
║     ██      ██   ██ ██████  ██      ██ ███████    ██    ██   ██ ║
║                                                                  ║
║                   ▒█▀▀█ █▀▀ █▀▀█ █▀▀█ █▀▀                    ║
║                   ▒█▄▄█ █   █▄▄▀ █▄▄▀ █▀▀                    ║
║                                                                  ║
╚══════════════════════════════════════════════════════════════════╝
```

> **Prometheus OS is not another Linux distribution.**  
> It is a complete reimagining of the operating system where **the AI is the interface**.  
> Applications become tools used by the AI. The user communicates with Prometheus instead of manually managing software.

---

## 📋 Table of Contents

- [🚀 Quick Start](#-quick-start)
- [🏗 Architecture](#-architecture)
- [🧠 AI Core](#-ai-core)
- [🖥 Desktop Environments](#-desktop-environments)
- [📦 Core Components](#-core-components)
- [📊 Performance](#-performance)
- [🛡 Security](#-security)
- [🔧 Development](#-development)
- [🤖 Robotics](#-robotics)
- [📁 Project Structure](#-project-structure)

---

## 🚀 Quick Start

```bash
# ─── Build from source ─────────────────────────────────────
git clone https://github.com/sudorootvector/prometheus-os.git
cd prometheus-os

# Build all core components
make all

# Build GNOME Desktop integration
make gnome

# ─── Install ───────────────────────────────────────────────
make install                # Core components
make install-gnome          # GNOME integration (optional)

# ─── Or build a bootable ISO ──────────────────────────────
make iso

# ─── Run ───────────────────────────────────────────────────
prometheus-bootstrap        # First-time system setup
reboot                      # Select "Prometheus" at login

# Or run in a session:
make run                    # Native Prometheus compositor
make run-gnome              # GNOME with Prometheus AI
```

<details>
<summary><b>📦 Arch Linux Package Installation</b></summary>

```bash
# Install from PKGBUILD
cd packaging/pkgbuilds/prometheus-meta
makepkg -si
prometheus-bootstrap
systemctl enable gdm
reboot
```
</details>

---

## 🏗 Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                     USER INTERFACE LAYER                          │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────────┐    │
│  │ Terminal │ │  Files   │ │ Settings │ │    Dashboard     │    │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └───────┬──────────┘    │
│       │            │            │               │                │
│  ┌────┴────────────┴────────────┴───────────────┴──────────┐     │
│  │                   DESKTOP SHELL                          │     │
│  │      Panel · Launcher · Notifications · Workspaces       │     │
│  └───────────────────────┬─────────────────────────────────┘     │
│                          │                                       │
│  ┌──────────────────────┴─────────────────────────────────┐      │
│  │              COMPOSITOR (wlroots-based)                  │      │
│  │   GPU Renderer · Layout Engine · Effects · Gestures     │      │
│  │          240 FPS · Physics Animations · Blur            │      │
│  └───────────────────────┬─────────────────────────────────┘      │
├──────────────────────────┼───────────────────────────────────────┤
│  ┌──────────────────────┴─────────────────────────────────┐      │
│  │                   AI CORE                                │      │
│  │                                                          │      │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐    │      │
│  │  │Reasoning │ │  Memory  │ │  Vision  │ │  Voice   │    │      │
│  │  │ Engine   │ │  Graph   │ │  Engine  │ │  Engine  │    │      │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘    │      │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐    │      │
│  │  │Automation│ │ Context  │ │  Plugin  │ │  Agents  │    │      │
│  │  │ Engine   │ │ Manager  │ │  System  │ │  Manager │    │      │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘    │      │
│  └───────────────────────┬─────────────────────────────────┘      │
├──────────────────────────┼───────────────────────────────────────┤
│  ┌──────────────────────┴─────────────────────────────────┐      │
│  │                 SECURITY LAYER                          │      │
│  │  Sandbox · Permissions · Audit · Encryption · AppArmor │      │
│  └───────────────────────┬─────────────────────────────────┘      │
├──────────────────────────┼───────────────────────────────────────┤
│  ┌──────────────────────┴─────────────────────────────────┐      │
│  │               ARCH LINUX FOUNDATION                     │      │
│  │  systemd · Wayland · PipeWire · Btrfs · Mesa · linux-zen│     │
│  │  pacman · AUR · Flatpak · SecureBoot · UEFI            │      │
│  └──────────────────────────────────────────────────────────┘      │
└──────────────────────────────────────────────────────────────────┘
```

---

## 🧠 AI Core

```
                        ┌─────────────────────┐
                        │   USER INPUT          │
                        │  Voice · Keyboard     │
                        └──────────┬──────────┘
                                   │
                        ┌──────────▼──────────┐
                        │   COMMAND ROUTER     │
                        └──────────┬──────────┘
                                   │
              ┌────────────────────┼────────────────────┐
              │                    │                    │
     ┌────────▼───┐      ┌────────▼───────┐   ┌───────▼───────┐
     │  REASONING │      │    MEMORY      │   │    VISION     │
     │   ENGINE   │      │    GRAPH       │   │    ENGINE     │
     │            │      │                │   │               │
     │ • Planning │      │ • Knowledge    │   │ • Screen cap  │
     │ • ReAct    │      │ • Embeddings   │   │ • OCR         │
     │ • TreeThought│    │ • Semantic     │   │ • Object det  │
     │ • Multi-agent│   │   search       │   │ • UI analysis │
     └────────┬───┘      └────────┬───────┘   └───────┬───────┘
              │                    │                    │
     ┌────────▼───┐      ┌────────▼───────┐   ┌───────▼───────┐
     │   VOICE    │      │  AUTOMATION    │   │   CONTEXT     │
     │   ENGINE   │      │   ENGINE       │   │   MANAGER     │
     │            │      │                │   │               │
     │ • STT      │      │ • Pattern rec  │   │ • Active apps │
     │ • TTS      │      │ • Workflow     │   │ • Resources   │
     │ • Wake word│      │ • Suggestions  │   │ • History     │
     └────────┬───┘      └────────┬───────┘   └───────┬───────┘
              │                    │                    │
              └────────────────────┼────────────────────┘
                                   │
                        ┌──────────▼──────────┐
                        │   ACTION EXECUTOR    │
                        └──────────┬──────────┘
                                   │
                        ┌──────────▼──────────┐
                        │    OS SUBSYSTEMS     │
                        └─────────────────────┘
```

### Capabilities

| Engine | Function | Status |
|--------|----------|--------|
| **Reasoning** | Multi-step planning, ReAct, Tree-of-Thought | ✅ |
| **Memory Graph** | Persistent knowledge graph with semantic search | ✅ |
| **Vision** | Screen capture, OCR, UI element detection | ✅ |
| **Voice** | Wake-word activation, STT, TTS | ✅ |
| **Automation** | Pattern recognition, workflow suggestions | ✅ |
| **Context** | Real-time system awareness, activity tracking | ✅ |
| **Plugin System** | Extensible via Python, Rust, C++, JS | ✅ |
| **Multi-Agent** | Collaborative task decomposition | ✅ |

---

## 🖥 Desktop Environments

### Option 1: Prometheus Native (Custom wlroots Compositor)

```yaml
Compositor: Prometheus-Compositor (wlroots)
Rendering:  GPU-accelerated, 240 FPS
Effects:    Blur, glow, shadows, rounded corners
Layout:     Dynamic tiling + floating, 5 modes
Workspaces: 9 with physics-based switching
Animations: Spring physics engine
```

### Option 2: GNOME Desktop (Fully Integrated)

```yaml
Shell:      GNOME 45+ with Prometheus dark theme
Compositor: Mutter tuned for 240 FPS, RT scheduling
Theme:      Dark glassmorphism with electric-blue accents
Extensions: AI indicator, Live dashboard, Dynamic tiling
Login:      Custom GDM theme with Prometheus branding
```

<details>
<summary><b>🖼 GNOME Theme Preview</b></summary>

```
┌─────────────────────────────────────────────────────────────┐
│  🔥 Prometheus AI │  ⟳ 12%  │  ⬡ 4.2G  │  ◉ 45°  │  ◆  │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ┌─────────────────────────────────────────────────────┐   │
│   │                                                     │   │
│   │   Window content with 12px rounded corners          │   │
│   │   and subtle glow effect on focus                   │   │
│   │                                                     │   │
│   │   ┌─────────────────────────────────────────────┐   │   │
│   │   │  Terminal with transparency and blur        │   │   │
│   │   └─────────────────────────────────────────────┘   │   │
│   └─────────────────────────────────────────────────────┘   │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  Workspace: main  dev  web  media  comm  files  ai  games  │
└─────────────────────────────────────────────────────────────┘
```
</details>

---

## 📊 Performance

<p align="center">
  <b>Idle RAM: < 900 MB</b> &nbsp;·&nbsp;
  <b>Boot: < 5 seconds</b> &nbsp;·&nbsp;
  <b>Compositor: 240 FPS</b> &nbsp;·&nbsp;
  <b>AI Response: < 100 ms</b>
</p>

```
Target Hardware
┌─────────────────────────────────────────────────────────────────┐
│  CPU  ████████████████████████████████████████░░░░  Intel 2nd+  │
│  RAM  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░  4 GB min    │
│  GPU  ████████████████████████████████████████████  Vulkan 1.3  │
│  DISK ████████████████████████████████████████████  64 GB SSD   │
└─────────────────────────────────────────────────────────────────┘

OS Optimization Targets
┌─────────────────────────────────────────────────────────────────┐
│  Boot Time      █████████████████████████████████████░░░░  5s  │
│  Wake Time      ████████████████████████████████████████  500ms│
│  App Launch     ████████████████████████████████████████  200ms│
│  Input Latency  ████████████████████████████████████████  5ms  │
│  Frame Time     ████████████████████████████████████████  4ms  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🛡 Security

| Layer | Technology | Status |
|-------|-----------|--------|
| **Boot** | UEFI Secure Boot with custom keys | ✅ |
| **Kernel** | Lockdown mode (integrity) | ✅ |
| **Sandbox** | Bubblewrap + Landlock + Namespaces | ✅ |
| **Permissions** | Per-application read/write/network/audio/video | ✅ |
| **Audit** | Full action logging with tamper evidence | ✅ |
| **Memory** | AES-256-GCM encryption for sensitive data | ✅ |
| **AI Control** | User approval required for destructive ops | ✅ |

---

## 📦 Core Components

```
prometheus-os/
├── compositor/          # wlroots-based Wayland compositor
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── compositor.rs # Core compositor loop
│   │   ├── config.rs    # TOML-based configuration
│   │   ├── render.rs    # GPU rendering pipeline
│   │   ├── layout.rs    # Dynamic tiling engine
│   │   ├── workspace.rs # 9-workspace manager
│   │   ├── input.rs     # Keyboard/mouse/touch/gestures
│   │   ├── effects.rs   # Blur, glow, shadow effects
│   │   └── shell.rs     # Panel integration
│
├── ai-core/             # Operating system intelligence
│   ├── src/
│   │   ├── engine.rs    # Main AI loop & command routing
│   │   ├── memory.rs    # Knowledge graph with embeddings
│   │   ├── reasoning.rs # Planning & multi-step reasoning
│   │   ├── vision.rs    # Screen capture & analysis
│   │   ├── voice.rs     # Speech-to-text & text-to-speech
│   │   ├── automation.rs# Pattern recognition & workflows
│   │   ├── context.rs   # System state tracking
│   │   └── plugin.rs    # Plugin architecture
│
├── desktop/             # Desktop environment
│   ├── src/
│   │   ├── panel.rs     # Top bar with AI integration
│   │   └── launcher.rs  # Universal AI-powered search
│   └── gnome/           # GNOME Desktop integration
│       ├── shell-theme/  # Dark glassmorphism GNOME Shell theme
│       ├── extensions/   # AI, Dashboard, Layout extensions
│       ├── mutter/       # 240 FPS compositor config
│       ├── gdm/          # Custom GDM login theme
│       ├── session/      # Prometheus GNOME session
│       └── config/       # dconf defaults
│
├── applications/        # Native applications (10 apps)
│   ├── terminal/        # AI-integrated terminal emulator
│   ├── files/           # File manager with smart search
│   ├── settings/        # System configuration center
│   ├── dashboard/       # AI command center & system monitor
│   ├── package-manager/ # pacman + AUR + Flatpak GUI
│   ├── system-monitor/  # Real-time resource monitor
│   └── snapshot-manager/# Btrfs snapshot management
│
├── security/            # Security subsystem
│   ├── sandbox.rs       # Bubblewrap sandboxing
│   ├── permissions.rs   # Capability-based permissions
│   ├── audit.rs         # Comprehensive audit logging
│   └── encryption.rs    # Memory encryption
│
├── sdk/                 # Multi-language SDK
│   ├── rust/            # Native Rust SDK
│   ├── python/          # Python SDK
│   ├── cpp/             # C++ SDK
│   └── javascript/      # JavaScript SDK
│
├── boot/                # Boot infrastructure
│   ├── systemd/         # Service files
│   └── initramfs/       # Btrfs rollback hooks
│
├── packaging/           # Build & packaging
│   ├── pkgbuilds/       # Arch Linux PKGBUILDs
│   └── iso/             # ISO builder
│
└── docs/                # Documentation
    ├── ARCHITECTURE.md  # System architecture
    ├── AI_CORE.md       # AI subsystem docs
    ├── DESKTOP.md       # Desktop environment docs
    ├── SECURITY.md       # Security model
    ├── PERFORMANCE.md   # Performance targets
    └── ROBOTICS.md      # Robotics integration
```

---

## 🔧 Development

```bash
# Build everything
make all              # Core components (Rust)
make sdk              # SDKs
make gnome            # GNOME integration
make iso              # Bootable ISO

# Test & lint
make test             # Run all tests
make lint             # Clippy lints

# Install
make install          # Core to /usr
make install-gnome    # GNOME integration
```

### SDK Usage

<details>
<summary><b>Rust</b></summary>

```rust
use prometheus_sdk::PrometheusSDK;

fn main() {
    let sdk = PrometheusSDK::new();
    let response = sdk.ai().query("What's on my screen?");
    sdk.desktop().send_notification("Hello", "From Prometheus!");
    let cpu = sdk.system().cpu_info();
}
```
</details>

<details>
<summary><b>Python</b></summary>

```python
from prometheus_sdk import PrometheusSDK

sdk = PrometheusSDK()
result = sdk.ai.query("Analyze my system")
notification = sdk.desktop.send_notification("AI", "Analysis complete")
cpu = sdk.system.cpu_usage()
```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { PrometheusSDK } = require('prometheus-sdk');
const sdk = new PrometheusSDK();
const response = await sdk.ai.query('What is on my screen?');
```
</details>

---

## 🤖 Robotics

```
┌─────────────────────────────────────────────────────────────┐
│                   PROMETHEUS ROBOTICS STACK                  │
├─────────────────────────────────────────────────────────────┤
│  ROS2  │  CAN Bus  │  Serial  │  GPIO  │  USB  │  PWM      │
├─────────┴───────────┴──────────┴────────┴───────┴──────────┤
│  LiDAR · Depth Cameras · IMU · GPS · Microphones           │
│  Robotic Arms · Drones · Servos · Stepper Motors           │
├─────────────────────────────────────────────────────────────┤
│  AI Perception · Path Planning · Control · SLAM            │
└─────────────────────────────────────────────────────────────┘
```

---

## 📊 Project Stats

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language            Files     Lines      % of project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Rust                 132      9,796      100.0%
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                132      9,796      100.0%
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Components:  15 crate workspace
Authors:     Prometheus OS Developers
License:     GNU General Public License v3
```

---

## 📜 License

**Prometheus OS** — Copyright © 2024 Prometheus OS Developers

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

---

<p align="center">
  <sub>
    Built with 🔥 by the Prometheus OS Team<br>
    <em>"The best way to predict the future is to invent it."</em>
  </sub>
</p>

<p align="center">
  <a href="https://github.com/sudorootvector/prometheus-os">
    <img src="https://img.shields.io/github/stars/sudorootvector/prometheus-os?style=social" alt="Stars">
  </a>
  <a href="https://github.com/sudorootvector/prometheus-os/fork">
    <img src="https://img.shields.io/github/forks/sudorootvector/prometheus-os?style=social" alt="Forks">
  </a>
</p>
