# Quick Start

Get Prometheus OS up and running in minutes.

## Prerequisites

- **Hardware:** x86_64, UEFI, 4 GB RAM, 20 GB storage
- **Host OS:** Arch Linux (for building from source)
- **Packages:** `base-devel`, `rust`, `cargo`, `wlroots`, `wayland-protocols`

## Installation Methods

=== "From Source"

    ```bash
    git clone https://github.com/Dev-serpent/prometheus-os.git
    cd prometheus-os
    make all
    sudo make install
    ```

=== "Bootable ISO"

    ```bash
    make iso
    # Write to USB:
    dd if=prometheus-os-*.iso of=/dev/sdX bs=4M status=progress
    ```

=== "Arch Package"

    ```bash
    # Via AUR helper
    yay -S prometheus-meta
    ```

## Post-Install

```bash
# Enable AI services
systemctl enable --now prometheus-ai.service

# For GNOME integration
prometheus-setup-gnome

# Verify installation
prometheus-dashboard
```

## First Boot

1. Select **Prometheus OS** at the boot menu
2. The Plymouth splash displays the Prometheus logo
3. The AI Core initializes automatically
4. The desktop environment loads within seconds
5. Press ++super+space++ to open the AI prompt

## Verify Installation

```bash
prometheus-dashboard          # Open the AI Command Center
prometheus-system-monitor     # Real-time resource monitor
prometheus-ai --query "status"  # Check AI Core health
```

## Next Steps

- [Explore the Architecture](architecture/index.md)
- [Read the AI Core Docs](ai-core/index.md)
- [Build an Application with the SDK](sdk/index.md)
- [Contribute to Prometheus OS](developer/contribute.md)
