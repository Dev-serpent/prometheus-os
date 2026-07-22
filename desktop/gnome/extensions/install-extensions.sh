#!/usr/bin/env bash
# Prometheus OS GNOME Shell Extension Installer
set -euo pipefail

PROMETHEUS_EXT_DIR="/usr/share/gnome-shell/extensions"

install_extension() {
    local name="$1"
    local src_dir="$(dirname "$0")/$name"
    local dest_dir="$PROMETHEUS_EXT_DIR/$name@prometheus-os.dev"

    if [ ! -d "$src_dir" ]; then
        echo "Extension source not found: $src_dir"
        return 1
    fi

    mkdir -p "$dest_dir"
    cp -r "$src_dir"/* "$dest_dir/"
    chmod -R 755 "$dest_dir"

    # Compile schemas if present
    if [ -d "$dest_dir/schemas" ]; then
        glib-compile-schemas "$dest_dir/schemas"
    fi

    echo "Installed: $name"
}

echo "Installing Prometheus OS GNOME Extensions..."
install_extension "prometheus-ai"
install_extension "prometheus-dashboard"
install_extension "prometheus-layout"
install_extension "prometheus-workspace"

# Enable extensions for all users
echo "Enabling Prometheus extensions..."
EXTENSIONS="prometheus-ai@prometheus-os.dev,prometheus-dashboard@prometheus-os.dev,prometheus-layout@prometheus-os.dev,prometheus-workspace@prometheus-os.dev"

for user_home in /home/*; do
    user=$(basename "$user_home")
    gsettings set org.gnome.shell enabled-extensions "$EXTENSIONS" 2>/dev/null || true
done

# Also set for future users
mkdir -p /etc/skel/.local/share/gnome-shell
echo "$EXTENSIONS" > /etc/skel/.local/share/gnome-shell/enabled-extensions.txt

echo "Prometheus GNOME Extensions installed and enabled."
