<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/Dev-serpent/prometheus-os/main/resources/brand/prometheus-logo-light.svg">
    <img src="https://raw.githubusercontent.com/Dev-serpent/prometheus-os/main/resources/brand/prometheus-logo-dark.svg" width="280" alt="Prometheus OS">
  </picture>
</p>

<h1 align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://readme-typing-svg.demolab.com?font=Inter&weight=700&size=28&duration=4000&pause=500&color=0078FF&center=true&vCenter=true&repeat=true&width=500&lines=AI-Native+Operating+System;15+Rust+Crates+%C2%B7+Zero+Runtime+Overhead;The+OS+is+the+AI">
    <img src="https://readme-typing-svg.demolab.com?font=Inter&weight=700&size=28&duration=4000&pause=500&color=0078FF&center=true&vCenter=true&repeat=true&width=500&lines=AI-Native+Operating+System;15+Rust+Crates+%C2%B7+Zero+Runtime+Overhead;The+OS+is+the+AI">
  </picture>
</h1>

<p align="center">
  <a href="https://github.com/Dev-serpent/prometheus-os"><img src="https://img.shields.io/badge/build-passing-0078FF?style=flat&logo=github" alt="Build"></a>
  <a href="https://github.com/Dev-serpent/prometheus-os/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-GPLv3-1A1A1A?style=flat" alt="License"></a>
  <a href="#"><img src="https://img.shields.io/badge/kernel-linux--zen-1A1A1A?style=flat&logo=linux" alt="Kernel"></a>
  <a href="#"><img src="https://img.shields.io/badge/display-Wayland-1A1A1A?style=flat" alt="Display"></a>
  <a href="#"><img src="https://img.shields.io/badge/language-Rust-1A1A1A?style=flat&logo=rust" alt="Language"></a>
  <a href="#"><img src="https://img.shields.io/badge/crates-15-1A1A1A?style=flat" alt="Crates"></a>
  <a href="#"><img src="https://img.shields.io/badge/ram_idle-%3C900MB-1A1A1A?style=flat" alt="RAM"></a>
</p>

---

Prometheus OS is an **AI-native operating system** architected from the kernel up — the AI is not an application running on the OS, it *is* the OS. Every subsystem from the compositor to the memory manager is designed to enable zero-latency human-AI interaction.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  User → Natural Language / Gesture / Voice                       │
├─────────────────────────────────────────────────────────────────┤
│  AI Core    │  Reasoning · Memory Graph · Vision · Voice · Auto  │
│  Security   │  Sandbox · Permissions · Audit · Memory Encryption │
│  Compositor │  wlroots · 240 FPS · Vulkan 1.3 · Blur/Glow       │
│  Desktop    │  Panel · Launcher · Notifications · 9 Workspaces   │
│  Apps       │  10 native Rust applications                       │
│  SDK        │  Rust · Python · C++ · JavaScript                  │
│  Kernel     │  linux-zen · Btrfs · systemd · Wayland · PipeWire  │
└─────────────────────────────────────────────────────────────────┘
```

## Core Stack

| Component | Language | Lines | Status |
|-----------|----------|-------|--------|
| **AI Core** — Cognitive engine, semantic memory graph, automation | Rust | ~1,800 | Active |
| **Compositor** — wlroots, Vulkan, blur/glow/shadow effects | Rust | ~1,200 | Active |
| **Desktop** — Panel, launcher, notification center | Rust | ~600 | Active |
| **Security** — Bubblewrap sandbox, audit, memory encryption | Rust | ~350 | Active |
| **10 Applications** — Terminal, files, browser, settings, dashboard, etc. | Rust | ~3,200 | Active |
| **SDK** — Multi-language bindings (Rust, Python, C++, JS) | Mixed | ~1,500 | Active |
| **GNOME Integration** — Theme, GDM, 3 Shell extensions, Mutter tuning | CSS/JS | ~2,000 | Stable |
| **Robotics** — ROS2, CAN, Serial, GPIO | Rust | ~200 | Experimental |

## AI Engine

```
Command → Intent Parser → Memory Graph Lookup → Context Builder
         → ReAct Reasoning Chain → Tool Selection → Action
         → Observation → Memory Update → Response
```

- **Memory Graph**: Persistent knowledge base with semantic search (DashMap + embeddings)
- **ReAct Loop**: Structured reasoning → action → observation cycle
- **Vision**: Screen capture, OCR, UI element detection, context analysis
- **Voice**: Wake-word activation, STT, TTS with configurable backends
- **Automation**: Pattern detection, workflow learning, autonomous execution
- **Plugin System**: Dynamic loading of capability extensions

## Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| Idle RAM | < 900 MB | ~1.2 GB (optimizing) |
| Cold boot | < 5 s | ~8 s (kernel init) |
| Compositor | 240 FPS | 144 FPS (wlroots cap) |
| AI response | < 100 ms | ~45 ms (local model) |
| Suspend wake | < 500 ms | ~300 ms |

## Build

```bash
git clone https://github.com/Dev-serpent/prometheus-os.git
cd prometheus-os

make all           # Build all 15 Rust crates
make gnome         # Build GNOME integration
make install       # Install to /usr (DESTDIR supported)
make iso           # Bootable Arch ISO
```

**Requirements:** Rust 1.75+, Wayland development libraries, base-devel

```
crates: 15  │  src files: 140+  │  lines: ~9,800  │  deps: 18 workspace-shared
```

## Desktop Environments

**Prometheus Native** — Custom wlroots compositor with GPU scheduling, dynamic tiling, physics animations, 9 workspaces, blur/glow/shadow. No traditional window manager.

**GNOME Session** — Full dark glassmorphism theme, 3 AI-integrated Shell extensions (AI indicator, live dashboard, dynamic tiling), custom GDM login screen, 240 Hz Mutter tuning with real-time scheduling.

## Applications

| App | Function |
|-----|----------|
| **Terminal** | GPU-accelerated terminal emulator |
| **Files** | AI-assisted file manager with semantic search |
| **Browser** | Web engine with AI page analysis |
| **Settings** | 8-module configuration center |
| **Dashboard** | Real-time AI command center with live monitoring |
| **Package Manager** | pacman + AUR + Flatpak unified interface |
| **System Monitor** | Color-coded /proc-based resource monitor |
| **Snapshot Manager** | Btrfs snapshot management with AI recovery |
| **Plugin Manager** | Extend AI capabilities at runtime |
| **Developer Hub** | SDK docs, API browser, plugin scaffold |

## Security Model

- **Sandbox**: Every application runs in a bubblewrap container with Landlock LSM
- **Permissions**: AI actions are gated by user-configured permission profiles
- **Audit**: All AI-initiated operations are logged with cryptographic integrity
- **Memory**: Runtime AES-256-GCM encryption for sensitive data
- **Boot**: Secure Boot with sbctl, measured boot, TPM 2.0 support
- **Rollback**: mkinitcpio hook for automatic snapshot rollback on boot failure

## SDK

```rust
use prometheus_sdk::prelude::*;

let prom = PrometheusSDK::new();
let response = prom.ai().query("Analyze my system resources").await?;
println!("{}", response);
```

```python
from prometheus_sdk import PrometheusSDK

sdk = PrometheusSDK()
result = sdk.ai.query("Show my top processes")
print(result)
```

Bindings: Rust (native) · Python · C++ · JavaScript

## Project

```
140+ source files  ·  9,800+ lines  ·  15 Cargo workspace members  ·  GPLv3
```

<p align="center">
  <sub>Built with Rust</sub>
</p>
