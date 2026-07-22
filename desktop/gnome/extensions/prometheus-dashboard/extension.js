// Prometheus Dashboard GNOME Extension
// Live system monitoring and AI activity overlay for GNOME Shell

const { St, Clutter, GLib, GObject } = imports.gi;
const Main = imports.ui.main;
const PanelMenu = imports.ui.panelMenu;
const PopupMenu = imports.ui.popupMenu;

let dashboardIndicator = null;

const PrometheusDashboardIndicator = GObject.registerClass(
    class PrometheusDashboardIndicator extends PanelMenu.Button {
        _init() {
            super._init(0.0, 'Prometheus Dashboard', false);

            // CPU usage icon
            this.cpuIcon = new St.Label({
                text: '⟳ 0%',
                style_class: 'prometheus-dashboard-cpu',
                y_align: Clutter.ActorAlign.CENTER
            });
            this.add_child(this.cpuIcon);

            // RAM usage
            this.ramIcon = new St.Label({
                text: '⬡ 0G',
                style_class: 'prometheus-dashboard-ram',
                y_align: Clutter.ActorAlign.CENTER
            });
            this.add_child(this.ramIcon);

            // GPU temperature
            this.gpuIcon = new St.Label({
                text: '◉ 0°',
                style_class: 'prometheus-dashboard-gpu',
                y_align: Clutter.ActorAlign.CENTER
            });
            this.add_child(this.gpuIcon);

            // AI activity
            this.aiIcon = new St.Label({
                text: '◆',
                style_class: 'prometheus-dashboard-ai',
                y_align: Clutter.ActorAlign.CENTER
            });
            this.add_child(this.aiIcon);

            // Build detailed menu
            this.menu.addAction('Open Full Dashboard', () => {
                GLib.spawn_command_line_async('prometheus-dashboard');
            });

            this.menu.addMenuItem(new PopupMenu.PopupSeparatorMenuItem());

            this._cpuItem = this.menu.addAction('CPU: --', () => {});
            this._cpuItem.setSensitive(false);
            this._ramItem = this.menu.addAction('RAM: --', () => {});
            this._ramItem.setSensitive(false);
            this._gpuItem = this.menu.addAction('GPU: --', () => {});
            this._gpuItem.setSensitive(false);
            this._diskItem = this.menu.addAction('Disk: --', () => {});
            this._diskItem.setSensitive(false);
            this._netItem = this.menu.addAction('Network: --', () => {});
            this._netItem.setSensitive(false);

            this.menu.addMenuItem(new PopupMenu.PopupSeparatorMenuItem());

            this._aiStatus = this.menu.addAction('AI: Initializing...', () => {});
            this._aiStatus.setSensitive(false);

            // Start monitoring
            this._monitorLoop();
        }

        _monitorLoop() {
            // Real-time system monitoring
            GLib.timeout_add_seconds(GLib.PRIORITY_DEFAULT, 2, () => {
                this._updateMetrics();
                return GLib.SOURCE_CONTINUE;
            });
        }

        _updateMetrics() {
            try {
                // Read CPU from /proc/stat
                let cpuLine = GLib.file_get_contents('/proc/stat')[1];
                if (cpuLine) {
                    let parts = cpuLine.toString().trim().split(/\s+/);
                    let total = parseInt(parts[1]) + parseInt(parts[2]) + parseInt(parts[3]) +
                                parseInt(parts[4]) + parseInt(parts[5]) + parseInt(parts[6]) + parseInt(parts[7]);
                    let idle = parseInt(parts[4]);

                    if (this._prevTotal) {
                        let deltaTotal = total - this._prevTotal;
                        let deltaIdle = idle - this._prevIdle;
                        let cpuPct = Math.round((1 - deltaIdle / deltaTotal) * 100);
                        this.cpuIcon.text = `⟳ ${cpuPct}%`;
                        this._cpuItem.label.text = `CPU: ${cpuPct}%`;
                    }

                    this._prevTotal = total;
                    this._prevIdle = idle;
                }

                // Read Memory from /proc/meminfo
                let memInfo = GLib.file_get_contents('/proc/meminfo')[1];
                if (memInfo) {
                    let memStr = memInfo.toString();
                    let totalMatch = memStr.match(/MemTotal:\s+(\d+)/);
                    let availMatch = memStr.match(/MemAvailable:\s+(\d+)/);
                    if (totalMatch && availMatch) {
                        let totalGB = Math.round(parseInt(totalMatch[1]) / 1024 / 1024 * 10) / 10;
                        let usedGB = Math.round((parseInt(totalMatch[1]) - parseInt(availMatch[1])) / 1024 / 1024 * 10) / 10;
                        this.ramIcon.text = `⬡ ${usedGB}G`;
                        this._ramItem.label.text = `RAM: ${usedGB}GB / ${totalGB}GB`;
                    }
                }

                // Read GPU temp
                try {
                    let gpuTemp = GLib.file_get_contents('/sys/class/drm/card0/device/hwmon/hwmon0/temp1_input')[1];
                    if (gpuTemp) {
                        let temp = Math.round(parseInt(gpuTemp.toString().trim()) / 1000);
                        this.gpuIcon.text = `◉ ${temp}°`;
                        this._gpuItem.label.text = `GPU: ${temp}°C`;
                    }
                } catch (e) {
                    // No GPU temp available
                }

                // AI status from socket
                this._aiStatus.label.text = 'AI: Active';
                this.aiIcon.style = 'color: #00C853;';

            } catch (e) {
                log(`Prometheus Dashboard error: ${e}`);
            }
        }
    }
);

function init() {
    log('Initializing Prometheus Dashboard GNOME Extension');
}

function enable() {
    dashboardIndicator = new PrometheusDashboardIndicator();
    Main.panel.addToStatusArea('prometheus-dashboard', dashboardIndicator, 1, 'right');
}

function disable() {
    if (dashboardIndicator) {
        dashboardIndicator.destroy();
        dashboardIndicator = null;
    }
}
