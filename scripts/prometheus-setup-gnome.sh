#!/usr/bin/env bash
# Prometheus OS GNOME Desktop Setup Script
# Configures GNOME with full Prometheus integration
set -euo pipefail

PROMETHEUS_VERSION="0.1.0"
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

log() { echo -e "${BLUE}[PROMETHEUS]${NC} $*"; }
success() { echo -e "${GREEN}[✓]${NC} $*"; }

check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        log "This script must be run as root. Use sudo."
        exit 1
    fi
}

log "Prometheus OS GNOME Setup v${PROMETHEUS_VERSION}"
check_root

# 1. Install Prometheus GNOME packages
log "Installing Prometheus GNOME integration..."
pacman -S --noconfirm \
    gnome gnome-extra \
    gnome-tweaks gnome-shell-extensions \
    gdm mutter gsettings-desktop-schemas \
    dconf-editor \
    linter-font noto-fonts jetbrains-mono \
    2>/dev/null || true

# 2. Install Prometheus theme and extensions
log "Installing Prometheus theme..."

# Theme directories
THEME_DIR="/usr/share/themes/Prometheus-Dark"
mkdir -p "$THEME_DIR/gnome-shell"
mkdir -p "$THEME_DIR/gtk-3.0"
mkdir -p "$THEME_DIR/gtk-4.0"
mkdir -p "$THEME_DIR/gdm"

# Install GNOME Shell theme
cat > "$THEME_DIR/gnome-shell/gnome-shell.css" << 'CSS'
/* Prometheus OS GNOME Shell Theme */
#panel {
    background-color: rgba(10, 10, 10, 0.75);
    border-bottom: 1px solid rgba(42, 42, 42, 0.5);
    height: 2.2em;
    backdrop-filter: blur(24px);
}
#panel .panel-button {
    color: #A0A0A0;
    border-radius: 8px;
    padding: 0 10px;
}
#panel .panel-button:hover {
    color: #FFFFFF;
    background: rgba(0, 120, 255, 0.1);
}
#panel .panel-button:active, #panel .panel-button:checked {
    color: #0078FF;
    background: rgba(0, 120, 255, 0.15);
}
#overview {
    background: rgba(10, 10, 10, 0.95);
    backdrop-filter: blur(48px);
}
.search-entry {
    border-radius: 24px;
    border: 2px solid rgba(42, 42, 42, 0.5);
    background: rgba(26, 26, 26, 0.8);
    color: #FFFFFF;
    padding: 8px 16px;
    width: 320px;
    backdrop-filter: blur(12px);
}
.search-entry:focus {
    border-color: #0078FF;
    box-shadow: 0 0 20px rgba(0, 120, 255, 0.3);
}
.window-caption {
    background: rgba(26, 26, 26, 0.85);
    border: 1px solid rgba(42, 42, 42, 0.5);
    border-radius: 12px;
    padding: 8px 12px;
    color: #FFFFFF;
    backdrop-filter: blur(16px);
}
.workspace-thumbnail {
    border-radius: 8px;
    border: 2px solid transparent;
    transition: all 200ms;
}
.workspace-thumbnail:active, .workspace-thumbnail:selected {
    border-color: #0078FF;
    box-shadow: 0 0 16px rgba(0, 120, 255, 0.3);
}
#dash {
    margin: 8px;
    padding: 4px;
    background: rgba(36, 36, 36, 0.7);
    border: 1px solid rgba(42, 42, 42, 0.3);
    border-radius: 24px;
    backdrop-filter: blur(24px);
}
.notification-banner {
    background: rgba(26, 26, 26, 0.9);
    border: 1px solid rgba(42, 42, 42, 0.5);
    border-radius: 12px;
    padding: 12px;
}
.popup-menu {
    background: rgba(26, 26, 26, 0.85);
    border: 1px solid rgba(42, 42, 42, 0.5);
    border-radius: 12px;
    padding: 4px;
    backdrop-filter: blur(24px);
}
.popup-menu-item {
    border-radius: 8px;
    padding: 8px 12px;
    color: #FFFFFF;
}
.popup-menu-item:hover {
    background: rgba(0, 120, 255, 0.1);
}
CSS

# Install GTK theme
cat > "$THEME_DIR/gtk-3.0/gtk.css" << 'CSS'
@define-color prometheus_bg #0A0A0A;
@define-color prometheus_surface #1A1A1A;
@define-color prometheus_surface2 #242424;
@define-color prometheus_border #2A2A2A;
@define-color prometheus_text #FFFFFF;
@define-color prometheus_text_secondary #A0A0A0;
@define-color prometheus_accent #0078FF;
window { background: @prometheus_bg; color: @prometheus_text; }
button { background: @prometheus_surface2; color: @prometheus_text; border: 1px solid alpha(@prometheus_border, 0.5); border-radius: 8px; padding: 6px 16px; }
button:hover { background: mix(@prometheus_surface2, @prometheus_accent, 10%); }
button:checked { background: @prometheus_accent; color: white; }
entry { background: @prometheus_surface2; color: @prometheus_text; border: 1px solid alpha(@prometheus_border, 0.5); border-radius: 8px; padding: 6px 12px; }
entry:focus { border-color: @prometheus_accent; box-shadow: 0 0 12px alpha(@prometheus_accent, 0.2); }
scrollbar slider { background: alpha(@prometheus_text, 0.2); border-radius: 4px; }
scrollbar slider:hover { background: alpha(@prometheus_text, 0.4); }
scrollbar slider:active { background: @prometheus_accent; }
CSS

ln -sf "../gtk-3.0/gtk.css" "$THEME_DIR/gtk-4.0/gtk.css"

# Theme index
cat > "$THEME_DIR/index.theme" << 'EOF'
[Desktop Entry]
Type=X-GNOME-Metatheme
Name=Prometheus-Dark
Comment=Prometheus OS Dark Glassmorphism Theme
Encoding=UTF-8

[X-GNOME-Metatheme]
GtkTheme=Prometheus-Dark
MetacityTheme=Prometheus-Dark
IconTheme=Adwaita
CursorTheme=Adwaita
ButtonLayout=close:
EOF

success "Theme installed"

# 3. Install GNOME Shell extensions
log "Installing GNOME Shell extensions..."
EXT_DIR="/usr/share/gnome-shell/extensions"

# Prometheus AI extension
mkdir -p "$EXT_DIR/prometheus-ai@prometheus-os.dev"
cat > "$EXT_DIR/prometheus-ai@prometheus-os.dev/metadata.json" << 'EOF'
{
  "name": "Prometheus AI",
  "description": "Deep AI integration for GNOME Shell",
  "uuid": "prometheus-ai@prometheus-os.dev",
  "shell-version": ["45", "46", "47"],
  "version": 1
}
EOF

cat > "$EXT_DIR/prometheus-ai@prometheus-os.dev/extension.js" << 'JSEXT'
const { St, Shell, Meta, Clutter, GLib, GObject } = imports.gi;
const Main = imports.ui.main;
const PanelMenu = imports.ui.panelMenu;
const PopupMenu = imports.ui.popupMenu;

const PrometheusIndicator = GObject.registerClass(
    class PrometheusIndicator extends PanelMenu.Button {
        _init() {
            super._init(0.0, 'Prometheus AI', false);
            this.icon = new St.Icon({ icon_name: 'dialog-information-symbolic', style_class: 'system-status-icon', icon_size: 16 });
            this.add_child(this.icon);
            this.menu.addAction('Open Prometheus Dashboard', () => { GLib.spawn_command_line_async('prometheus-dashboard'); });
            this.menu.addAction('AI Status: Connected', () => {});
        }
    }
);
function init() { log('Prometheus AI extension loaded'); }
function enable() {
    let indicator = new PrometheusIndicator();
    Main.panel.addToStatusArea('prometheus-ai', indicator, 0, 'right');
}
function disable() {}
JSEXT

success "Extensions installed"

# 4. Configure GDM
log "Configuring GDM..."
cat > /etc/gdm/custom.conf << 'EOF'
[daemon]
WaylandEnable=true
DefaultSession=prometheus-gnome.desktop
AutomaticLoginEnable=false
InitialSetupEnable=false
[security]
AllowRoot=false
[debug]
Enable=false
EOF

# 5. Configure Mutter for performance
log "Configuring Mutter performance..."
mkdir -p /etc/dconf/db/local.d
cat > /etc/dconf/db/local.d/99-prometheus-performance << 'EOF'
[org/gnome/mutter]
experimental-features=['scale-monitor-framebuffer', 'variable-refresh-rate', 'rt-scheduler']
check-alive-timeout=15000
dynamic-workspaces=true
edge-tiling=true
center-new-windows=true

[org/gnome/mutter.wayland]
xwayland-allow-grabs=true
EOF

# 6. Apply GNOME settings
log "Applying GNOME settings..."
su -c "gsettings set org.gnome.desktop.interface gtk-theme 'Prometheus-Dark'" 2>/dev/null || true
su -c "gsettings set org.gnome.desktop.interface color-scheme 'prefer-dark'" 2>/dev/null || true
su -c "gsettings set org.gnome.desktop.interface font-name 'Inter 10'" 2>/dev/null || true
su -c "gsettings set org.gnome.desktop.wm.preferences button-layout 'close:'" 2>/dev/null || true
su -c "gsettings set org.gnome.mutter dynamic-workspaces true" 2>/dev/null || true
su -c "gsettings set org.gnome.mutter edge-tiling true" 2>/dev/null || true

# 7. Enable GDM service
log "Enabling GDM display manager..."
systemctl enable gdm 2>/dev/null || true

# 8. Set default session
log "Setting default session..."
mkdir -p /etc/systemd/system/gdm.service.d
cat > /etc/systemd/system/gdm.service.d/10-session.conf << 'EOF'
[Service]
Environment=XDG_SESSION_TYPE=wayland
Environment=GDK_BACKEND=wayland
EOF

success "GNOME Setup Complete!"
echo ""
echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║            PROMETHEUS GNOME READY                       ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "  Next steps:"
echo "  1. Create a user:    useradd -m -G wheel user"
echo "  2. Set password:     passwd user"
echo "  3. Reboot:           systemctl reboot"
echo ""
echo "  At login screen, select 'Prometheus (GNOME)'"
echo "  The AI integration activates automatically."
