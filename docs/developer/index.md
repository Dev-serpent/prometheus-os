# Developer Guide

Welcome to Prometheus OS development. This guide covers everything from setting up your development environment to contributing production code.

## Prerequisites

- **Rust**: 1.75+ (install via `rustup`)
- **System packages**: `base-devel`, `wlroots`, `wayland-protocols`, `libxkbcommon`, `libinput`, `pkg-config`
- **GPU drivers**: Mesa (Intel/AMD) or NVIDIA drivers with Vulkan 1.3 support
- **Tools**: `clang`, `lld`, `cmake`, `git`

## Quick Setup

```bash
# Clone the repository
git clone https://github.com/Dev-serpent/prometheus-os.git
cd prometheus-os

# Build everything
make all

# Or build individual components
make compositor    # wlroots compositor
make ai            # AI Core
make desktop       # Desktop shell
make apps          # All applications

# Run tests
make test

# Run linter
make lint
```

## Project Map

```
prometheus-os/
├── ai-core/              # AI reasoning, memory, vision, voice
│   ├── src/
│   │   ├── engine.rs     # Async command loop
│   │   ├── memory.rs     # Knowledge graph
│   │   ├── reasoning.rs  # ReAct chain
│   │   ├── vision.rs     # Screen capture
│   │   ├── voice.rs      # STT/TTS
│   │   ├── automation.rs # Pattern detection
│   │   └── plugin.rs     # Plugin system
│   └── Cargo.toml
├── compositor/           # wlroots-based compositor
│   ├── src/
│   │   ├── main.rs       # Entry point
│   │   ├── render.rs     # Vulkan renderer
│   │   ├── effects.rs    # Blur/glow/shadow
│   │   ├── input.rs      # libinput handling
│   │   └── layout.rs     # Window management
│   └── Cargo.toml
├── desktop/              # Desktop shell
├── security/             # Sandbox, permissions, audit
├── applications/         # All 10 application crates
├── sdk/                  # Multi-language SDK
├── resources/            # Branding, themes, assets
├── boot/                 # Systemd units, initramfs
├── packaging/            # PKGBUILDs, ISO builder
├── docs/                 # Documentation (this site)
└── Cargo.toml            # Workspace root
```

## Development Workflow

```mermaid
graph LR
    A[Fork Repo] --> B[Create Branch]
    B --> C[Make Changes]
    C --> D[make test]
    D --> E[make lint]
    E --> F[Commit]
    F --> G[Push]
    G --> H[Open PR]
```

## Code Review

All contributions go through code review:

- Formatting: `rustfmt`
- Linting: `clippy` (all warnings treated as errors)
- Tests: all tests must pass
- Coverage: new code should maintain or improve coverage
- Documentation: public APIs must be documented

## Next Steps

- [Project Structure](structure.md) — Deep dive into the codebase
- [Coding Standards](standards.md) — Rust conventions and patterns
- [Building from Source](build.md) — Detailed build instructions
- [Contribution Guide](contribute.md) — How to contribute
- [Testing Guide](testing.md) — Running and writing tests
