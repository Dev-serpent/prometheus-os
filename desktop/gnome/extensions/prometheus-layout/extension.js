// Prometheus Dynamic Layout GNOME Extension
// Advanced window tiling and layout management for GNOME Mutter

const { Meta, Shell, St, Clutter, GObject, GLib } = imports.gi;
const Main = imports.ui.main;
const ExtensionUtils = imports.misc.extensionUtils;

let layoutManager = null;

const LayoutMode = {
    FLOATING: 0,
    MASTER_STACK: 1,
    GRID: 2,
    MONOCLE: 3,
    SPIRAL: 4
};

const PrometheusLayoutManager = GObject.registerClass(
    class PrometheusLayoutManager extends GObject.Object {
        _init() {
            super._init();
            this._mode = LayoutMode.MASTER_STACK;
            this._gap = 8;
            this._borderSize = 2;
            this._masterFactor = 0.6;
            this._monitors = {};
            this._keybindings = [];

            this._setupKeybindings();
            this._connectSignals();
        }

        _setupKeybindings() {
            // Toggle layout modes
            Main.wm.addKeybinding(
                'toggle-layout',
                new Meta.KeyBindingAction('toggle-layout'),
                Meta.KeyBindingFlags.NONE,
                Shell.ActionMode.NORMAL,
                () => this._cycleLayout()
            );

            // Focus navigation
            Main.wm.addKeybinding(
                'focus-left',
                new Meta.KeyBindingAction('focus-left'),
                Meta.KeyBindingFlags.NONE,
                Shell.ActionMode.NORMAL,
                () => this._focusDirection(Meta.MotionDirection.LEFT)
            );

            Main.wm.addKeybinding(
                'focus-right',
                new Meta.KeyBindingAction('focus-right'),
                Meta.KeyBindingFlags.NONE,
                Shell.ActionMode.NORMAL,
                () => this._focusDirection(Meta.MotionDirection.RIGHT)
            );

            Main.wm.addKeybinding(
                'focus-up',
                new Meta.KeyBindingAction('focus-up'),
                Meta.KeyBindingFlags.NONE,
                Shell.ActionMode.NORMAL,
                () => this._focusDirection(Meta.MotionDirection.UP)
            );

            Main.wm.addKeybinding(
                'focus-down',
                new Meta.KeyBindingAction('focus-down'),
                Meta.KeyBindingFlags.NONE,
                Shell.ActionMode.NORMAL,
                () => this._focusDirection(Meta.MotionDirection.DOWN)
            );

            // Move window
            Main.wm.addKeybinding(
                'move-left',
                new Meta.KeyBindingAction('move-left'),
                Meta.KeyBindingFlags.NONE,
                Shell.ActionMode.NORMAL,
                () => this._moveDirection(Meta.MotionDirection.LEFT)
            );

            Main.wm.addKeybinding(
                'move-right',
                new Meta.KeyBindingAction('move-right'),
                Meta.KeyBindingFlags.NONE,
                Shell.ActionMode.NORMAL,
                () => this._moveDirection(Meta.MotionDirection.RIGHT)
            );

            // Toggle float
            Main.wm.addKeybinding(
                'toggle-float',
                new Meta.KeyBindingAction('toggle-float'),
                Meta.KeyBindingFlags.NONE,
                Shell.ActionMode.NORMAL,
                () => this._toggleFloating()
            );
        }

        _connectSignals() {
            // Connect to window management signals
            global.display.connect('window-created', (display, window) => {
                this._onWindowCreated(window);
            });

            global.display.connect('window-entered-monitor', (display, monitor, window) => {
                this._arrangeMonitor(monitor);
            });

            global.display.connect('window-left-monitor', (display, monitor, window) => {
                this._arrangeMonitor(monitor);
            });

            global.display.connect('size-changed', () => {
                this._arrangeAll();
            });
        }

        _onWindowCreated(window) {
            // Skip special windows
            if (window.is_override_redirect() ||
                window.get_window_type() !== Meta.WindowType.NORMAL) {
                return;
            }

            // Apply window rules
            this._applyWindowRules(window);

            // Arrange after a short delay
            GLib.timeout_add(GLib.PRIORITY_DEFAULT, 100, () => {
                let monitor = window.get_monitor();
                this._arrangeMonitor(monitor);
                return GLib.SOURCE_REMOVE;
            });
        }

        _applyWindowRules(window) {
            let title = window.get_title();
            let wmClass = window.get_wm_class();

            // Application-specific rules
            const rules = {
                'firefox': { float: false },
                'Alacritty': { float: false },
                'org.gnome.Nautilus': { float: false },
                'org.gnome.Settings': { float: true },
                'gnome-calculator': { float: true },
                'gnome-control-center': { float: true }
            };

            let rule = rules[wmClass];
            if (rule) {
                if (rule.float) {
                    window.make_above();
                }
            }
        }

        _cycleLayout() {
            this._mode = (this._mode + 1) % 5;
            this._arrangeAll();

            // Show OSD notification
            const modeNames = ['Floating', 'Master & Stack', 'Grid', 'Monocle', 'Spiral'];
            Main.osdWindowManager.show(-1, `Layout: ${modeNames[this._mode]}`);
        }

        _arrangeAll() {
            for (let i = 0; i < global.display.get_n_monitors(); i++) {
                this._arrangeMonitor(i);
            }
        }

        _arrangeMonitor(monitorIndex) {
            let monitor = global.display.get_monitor_geometry(monitorIndex);
            let workspace = global.workspace_manager.get_active_workspace();
            let windows = workspace.list_windows().filter(w =>
                w.get_monitor() === monitorIndex &&
                !w.is_minimized() &&
                !w.is_override_redirect() &&
                !w.is_attached_dialog()
            );

            if (windows.length === 0) return;

            switch (this._mode) {
                case LayoutMode.FLOATING:
                    break;
                case LayoutMode.MASTER_STACK:
                    this._arrangeMasterStack(monitor, windows);
                    break;
                case LayoutMode.GRID:
                    this._arrangeGrid(monitor, windows);
                    break;
                case LayoutMode.MONOCLE:
                    this._arrangeMonocle(monitor, windows);
                    break;
                case LayoutMode.SPIRAL:
                    this._arrangeSpiral(monitor, windows);
                    break;
            }
        }

        _arrangeMasterStack(monitor, windows) {
            let gap = this._gap;
            let masterW = Math.round((monitor.width - gap * 2) * this._masterFactor);
            let stackW = monitor.width - masterW - gap * 3;

            if (windows.length === 1) {
                windows[0].move_resize_frame(
                    false, monitor.x + gap, monitor.y + gap,
                    monitor.width - gap * 2, monitor.height - gap * 2
                );
                return;
            }

            // Master window
            windows[0].move_resize_frame(
                false, monitor.x + gap, monitor.y + gap,
                masterW, monitor.height - gap * 2
            );

            // Stack windows
            let stackCount = windows.length - 1;
            let stackH = (monitor.height - gap * (stackCount + 1)) / stackCount;
            for (let i = 1; i < windows.length; i++) {
                let y = monitor.y + gap + (i - 1) * (stackH + gap);
                windows[i].move_resize_frame(
                    false, monitor.x + masterW + gap * 2, y,
                    stackW, Math.max(stackH, 100)
                );
            }
        }

        _arrangeGrid(monitor, windows) {
            let gap = this._gap;
            let cols = Math.ceil(Math.sqrt(windows.length));
            let rows = Math.ceil(windows.length / cols);
            let cellW = (monitor.width - gap * (cols + 1)) / cols;
            let cellH = (monitor.height - gap * (rows + 1)) / rows;

            for (let i = 0; i < windows.length; i++) {
                let col = i % cols;
                let row = Math.floor(i / cols);
                windows[i].move_resize_frame(
                    false,
                    monitor.x + gap + col * (cellW + gap),
                    monitor.y + gap + row * (cellH + gap),
                    cellW, cellH
                );
            }
        }

        _arrangeMonocle(monitor, windows) {
            // Focused window gets full screen, others minimized
            let focusWindow = global.display.get_focus_window();
            if (focusWindow && windows.includes(focusWindow)) {
                focusWindow.move_resize_frame(
                    false, monitor.x, monitor.y,
                    monitor.width, monitor.height
                );
                windows.forEach(w => {
                    if (w !== focusWindow) w.minimize();
                });
            }
        }

        _arrangeSpiral(monitor, windows) {
            // Fibonacci spiral layout
            let gap = this._gap;
            let x = monitor.x + gap;
            let y = monitor.y + gap;
            let w = monitor.width - gap * 2;
            let h = monitor.height - gap * 2;

            for (let i = 0; i < windows.length; i++) {
                windows[i].move_resize_frame(false, x, y, w, h);

                if (i % 2 === 0) {
                    w = Math.round(w * 0.618);
                    x += monitor.width - w - gap;
                } else {
                    h = Math.round(h * 0.618);
                    y += monitor.height - h - gap;
                }
            }
        }

        _focusDirection(direction) {
            let focusWindow = global.display.get_focus_window();
            if (!focusWindow) return;

            let workspace = global.workspace_manager.get_active_workspace();
            let windows = workspace.list_windows().filter(w =>
                w.get_monitor() === focusWindow.get_monitor() &&
                !w.is_minimized()
            );

            if (windows.length < 2) return;

            let focusRect = focusWindow.get_frame_rect();
            let centerX = focusRect.x + focusRect.width / 2;
            let centerY = focusRect.y + focusRect.height / 2;

            let best = null;
            let bestDist = Infinity;

            for (let w of windows) {
                if (w === focusWindow) continue;
                let rect = w.get_frame_rect();
                let wCenterX = rect.x + rect.width / 2;
                let wCenterY = rect.y + rect.height / 2;

                let dx = wCenterX - centerX;
                let dy = wCenterY - centerY;
                let dist = Math.sqrt(dx * dx + dy * dy);

                let inDirection = false;
                switch (direction) {
                    case Meta.MotionDirection.LEFT:
                        inDirection = dx < 0 && Math.abs(dy) < Math.abs(dx);
                        break;
                    case Meta.MotionDirection.RIGHT:
                        inDirection = dx > 0 && Math.abs(dy) < Math.abs(dx);
                        break;
                    case Meta.MotionDirection.UP:
                        inDirection = dy < 0 && Math.abs(dx) < Math.abs(dy);
                        break;
                    case Meta.MotionDirection.DOWN:
                        inDirection = dy > 0 && Math.abs(dx) < Math.abs(dy);
                        break;
                }

                if (inDirection && dist < bestDist) {
                    bestDist = dist;
                    best = w;
                }
            }

            if (best) {
                global.display.focus_window(best);
                best.activate(global.get_current_time());
            }
        }

        _moveDirection(direction) {
            let window = global.display.get_focus_window();
            if (!window) return;

            let rect = window.get_frame_rect();
            let step = 50;
            let x = rect.x, y = rect.y;

            switch (direction) {
                case Meta.MotionDirection.LEFT:
                    x -= step;
                    break;
                case Meta.MotionDirection.RIGHT:
                    x += step;
                    break;
                case Meta.MotionDirection.UP:
                    y -= step;
                    break;
                case Meta.MotionDirection.DOWN:
                    y += step;
                    break;
            }

            window.move_frame(false, x, y);
        }

        _toggleFloating() {
            let window = global.display.get_focus_window();
            if (!window) return;

            // Toggle between tiled and floating
            // Real implementation would save/restore geometry
            Main.osdWindowManager.show(-1, 'Toggled floating mode');
        }

        setMode(mode) {
            this._mode = mode;
            this._arrangeAll();
        }

        destroy() {
            // Cleanup keybindings
            Main.wm.removeKeybinding('toggle-layout');
            Main.wm.removeKeybinding('focus-left');
            Main.wm.removeKeybinding('focus-right');
            Main.wm.removeKeybinding('focus-up');
            Main.wm.removeKeybinding('focus-down');
            Main.wm.removeKeybinding('move-left');
            Main.wm.removeKeybinding('move-right');
            Main.wm.removeKeybinding('toggle-float');
        }
    }
);

function init() {
    log('Initializing Prometheus Dynamic Layout GNOME Extension');
}

function enable() {
    layoutManager = new PrometheusLayoutManager();
}

function disable() {
    if (layoutManager) {
        layoutManager.destroy();
        layoutManager = null;
    }
}
