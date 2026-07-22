#!/usr/bin/env bash
# Prometheus GNOME Session - Launches GNOME with Prometheus AI integration

export XDG_SESSION_TYPE=wayland
export XDG_CURRENT_DESKTOP=GNOME:Prometheus
export XDG_SESSION_DESKTOP=prometheus-gnome
export GNOME_SHELL_SESSION_MODE=prometheus

# Performance environment
export CLUTTER_BACKEND=wayland
export CLUTTER_DEFAULT_FPS=240
export MUTTER_DEBUG_NUM_LAYERS=512
export MUTTER_DEBUG_NUM_GRAB_OP_REGIONS=512

# Disable X11
export WAYLAND_DISPLAY=wayland-1
export MOZ_ENABLE_WAYLAND=1
export QT_QPA_PLATFORM=wayland
export GDK_BACKEND=wayland
export SDL_VIDEODRIVER=wayland
export _JAVA_AWT_WM_NONREPARENTING=1

# Fork Prometheus AI Core
/usr/bin/prometheus-ai &
PROMETHEUS_AI_PID=$!

# Fork Prometheus Desktop Services
/usr/bin/prometheus-dashboard --daemon &
/usr/bin/prometheus-automation-engine &

# Launch GNOME Session
exec gnome-session --session=prometheus-gnome "$@"
