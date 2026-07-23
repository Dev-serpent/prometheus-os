# CLI Reference

## System Commands

### `prometheus-dashboard`

AI Command Center — real-time system monitor and AI interface.

```bash
prometheus-dashboard [options]

Options:
  --refresh <sec>    Refresh interval (default: 2)
  --no-ai            Disable AI display
  --json             Output JSON for scripting
  --help             Show help
```

### `prometheus-ai`

AI Core CLI interface.

```bash
prometheus-ai [options] [query]

Options:
  --query "<text>"   Send a query to the AI
  --interactive      Start interactive session
  --status           Check AI Core health
  --reload           Reload configuration
  --model <name>     Specify model to use
  --json             JSON output
  --help             Show help

Examples:
  prometheus-ai --query "What's my CPU usage?"
  prometheus-ai --interactive
  prometheus-ai --status
```

### `prometheus-system-monitor`

Real-time resource monitor.

```bash
prometheus-system-monitor [options]

Options:
  --cpu-only         Show CPU information only
  --memory-only      Show memory information only
  --json             Output as JSON
  --interval <ms>    Update interval (default: 1000)
  --help             Show help
```

### `prometheus-settings`

System configuration tool.

```bash
prometheus-settings <module> [action]

Modules:
  display            Display configuration
  input              Input device configuration
  network            Network settings
  audio              Audio configuration
  ai                 AI Core settings
  security           Security configuration
  updates            Update settings
  system             System configuration

Actions:
  show               Show current configuration
  enable             Enable configuration
  disable            Disable configuration

Examples:
  prometheus-settings display show
  prometheus-settings ai enable
```

## Package Management

### `prometheus-package-manager`

```bash
prometheus-package-manager <command> [args]

Commands:
  search <query>     Search packages (pacman + AUR + Flatpak)
  install <pkg>      Install a package
  remove <pkg>       Remove a package
  list               List installed packages
  update             Check for updates
  upgrade            Full system upgrade
  clean              Clean package cache

Examples:
  prometheus-package-manager search firefox
  prometheus-package-manager install firefox
  prometheus-package-manager upgrade
```

## Snapshot Management

### `prometheus-snapshot-manager`

```bash
prometheus-snapshot-manager <command> [args]

Commands:
  list               List snapshots
  create [desc]      Create snapshot with optional description
  restore <num>      Restore a snapshot
  delete <num>       Delete a snapshot
  status             Show Btrfs status

Examples:
  prometheus-snapshot-manager list
  prometheus-snapshot-manager create "before-upgrade"
  prometheus-snapshot-manager restore 42
```

## Build System

### `make`

```bash
make all             # Build all components
make compositor      # Build compositor only
make desktop         # Build desktop shell
make ai              # Build AI Core
make apps            # Build all applications
make sdk             # Build SDK
make gnome           # Build GNOME integration
make install         # Install to system
make install-gnome   # Install GNOME integration
make iso             # Build bootable ISO
make run             # Run native compositor
make run-gnome       # Run GNOME session
make test            # Run all tests
make lint            # Run clippy
make clean           # Remove build artifacts
make help            # Show all targets
```

## Service Management

```bash
# AI Core service
systemctl status prometheus-ai.service
systemctl start prometheus-ai.service
systemctl enable prometheus-ai.service

# Compositor service (for native session)
systemctl status prometheus-compositor.service

# GNOME display manager
systemctl enable --now gdm.service
```
