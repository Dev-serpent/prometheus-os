// Prometheus AI GNOME Shell Extension
// Integrates Prometheus AI directly into the GNOME Shell

const { St, Shell, Meta, Clutter, GLib, Gio, GObject } = imports.gi;
const Main = imports.ui.main;
const PanelMenu = imports.ui.panelMenu;
const PopupMenu = imports.ui.popupMenu;
const Panel = imports.ui.panel;
const Util = imports.misc.util;
const MessageTray = imports.ui.messageTray;

let prometheusIndicator = null;
let aiSocket = null;
let aiSessionId = null;

const PrometheusIndicator = GObject.registerClass(
    class PrometheusIndicator extends PanelMenu.Button {
        _init() {
            super._init(0.0, 'Prometheus AI', false);

            // AI icon with glow effect
            this.icon = new St.Icon({
                icon_name: 'prometheus-ai-symbolic',
                style_class: 'prometheus-ai-icon',
                icon_size: 20
            });
            this.add_child(this.icon);

            // Status dot
            this.statusDot = new St.Widget({
                style_class: 'prometheus-ai-status',
                reactive: false,
                width: 8,
                height: 8
            });
            this.add_child(this.statusDot);

            // Build menu
            this.menu.addAction('Open Prometheus Dashboard', () => {
                Util.spawn(['prometheus-dashboard']);
            });

            this.menu.addAction('Ask Prometheus...', () => {
                this._showQuickQuery();
            });

            this.menu.addMenuItem(new PopupMenu.PopupSeparatorMenuItem());

            let analysisItem = this.menu.addAction('Analyze Screen', () => {
                this._analyzeScreen();
            });

            this.menu.addMenuItem(new PopupMenu.PopupSeparatorMenuItem());

            let autoItem = this.menu.addAction('View Automations', () => {
                Util.spawn(['prometheus-automation-viewer']);
            });

            let memoryItem = this.menu.addAction('Memory Graph', () => {
                Util.spawn(['prometheus-memory-explorer']);
            });

            this.menu.addMenuItem(new PopupMenu.PopupSeparatorMenuItem());

            this._statusItem = this.menu.addAction('AI Status: Active', () => {});
            this._statusItem.setSensitive(false);

            this._connectAI();
        }

        _connectAI() {
            // Unix socket connection to Prometheus AI Core
            try {
                aiSocket = new Gio.SocketClient();
                this._updateStatus('connected');
            } catch (e) {
                log('Prometheus AI: Connection failed - ' + e);
                this._updateStatus('disconnected');
            }
        }

        _updateStatus(state) {
            if (state === 'connected') {
                this.statusDot.style = 'background-color: #00C853; border-radius: 4px; box-shadow: 0 0 6px rgba(0,200,83,0.6);';
                this._statusItem.label.text = 'AI Status: Connected';
            } else {
                this.statusDot.style = 'background-color: #FF1744; border-radius: 4px;';
                this._statusItem.label.text = 'AI Status: Disconnected';
            }
        }

        _showQuickQuery() {
            let entry = new St.Entry({
                style_class: 'prometheus-ai-query-entry',
                hint_text: 'Ask Prometheus anything...',
                track_hover: true,
                can_focus: true
            });

            let modal = new ModalDialog(entry);
            // In a real extension, this would show a text input
            // and send the query to Prometheus AI Core
        }

        _analyzeScreen() {
            // Capture screen and send to Prometheus Vision Engine
            let screen = global.screen;
            let display = global.display;
            log('Prometheus AI: Analyzing screen...');
            // Real implementation would capture via PipeWire/Wayland
        }
    }
);

class ModalDialog {
    constructor(content) {
        // Modal overlay for AI queries
    }
}

function init() {
    log('Initializing Prometheus AI GNOME Extension v0.1.0');
}

function enable() {
    prometheusIndicator = new PrometheusIndicator();
    Main.panel.addToStatusArea('prometheus-ai', prometheusIndicator, 0, 'right');

    // Add AI glow to panel
    Main.panel.add_style_class_name('prometheus-ai-active');
}

function disable() {
    if (prometheusIndicator) {
        prometheusIndicator.destroy();
        prometheusIndicator = null;
    }
    Main.panel.remove_style_class_name('prometheus-ai-active');
}
