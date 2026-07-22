#!/usr/bin/env bash
set -euo pipefail

# Prometheus OS Bootstrap Script
# First-time setup after installation

PROMETHEUS_VERSION="0.1.0"
LOG_FILE="/var/log/prometheus-bootstrap.log"

log() {
    echo "[$(date '+%H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

error() {
    echo "[ERROR] $*" | tee -a "$LOG_FILE" >&2
    exit 1
}

check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        error "This script must be run as root"
    fi
}

log "=== Prometheus OS Bootstrap v${PROMETHEUS_VERSION} ==="
check_root

# 1. Configure system
log "Configuring system..."

# Set hostname
echo "prometheus" > /etc/hostname
hostname prometheus

# Set timezone to UTC
ln -sf /usr/share/zoneinfo/UTC /etc/localtime

# Generate locale
sed -i 's/^#en_US.UTF-8/en_US.UTF-8/' /etc/locale.gen
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf

# 2. Configure Btrfs snapshots
log "Configuring Btrfs snapshots..."
if command -v snapper &>/dev/null; then
    snapper -c root create-config /
    snapper -c home create-config /home
    systemctl enable snapper-timeline.timer
    systemctl enable snapper-cleanup.timer
    log "Snapper configured for root and home"
fi

# 3. Configure mkinitcpio
log "Configuring initramfs..."
cat > /etc/mkinitcpio.conf << 'EOF'
MODULES=(btrfs intel_agp i915 amdgpu nvidia nvidia_modeset nvidia_uvm nvidia_drm)
BINARIES=(/usr/bin/btrfs)
FILES=(/etc/prometheus/prometheus.conf)
HOOKS=(base udev autodetect modconf kms keyboard keymap consolefont block filesystems fsck)
EOF

mkinitcpio -P

# 4. Configure bootloader
log "Configuring systemd-boot..."
if [ -d /boot ]; then
    bootctl install

    # Prometheus boot entry
    mkdir -p /boot/loader/entries
    cat > /boot/loader/entries/prometheus.conf << 'BOOT'
title   Prometheus OS
linux   /vmlinuz-linux-zen
initrd  /intel-ucode.img
initrd  /amd-ucode.img
initrd  /initramfs-linux-zen.img
options root=LABEL=PROMETHEUS_ROOT rw rootflags=subvol=@ quiet splash loglevel=3 rd.systemd.show_status=auto rd.udev.log_level=3 vt.global_cursor_default=0 mitigations=off nowatchdog nmi_watchdog=0
BOOT

    # Recovery entry
    cat > /boot/loader/entries/prometheus-recovery.conf << 'BOOT'
title   Prometheus OS (Recovery Mode)
linux   /vmlinuz-linux-zen
initrd  /intel-ucode.img
initrd  /amd-ucode.img
initrd  /initramfs-linux-zen.img
options root=LABEL=PROMETHEUS_ROOT rw rootflags=subvol=@ mitigations=off systemd.unit=rescue.target
BOOT

    cat > /boot/loader/loader.conf << 'LOADER'
default  prometheus
timeout  0
console-mode max
editor   no
LOADER

    log "Bootloader configured"
fi

# 5. Configure users
log "Configuring users..."
if ! id -u prometheus &>/dev/null; then
    useradd -m -G wheel,audio,video,input,storage,power,network -s /bin/bash prometheus
    log "Created user 'prometheus'"
    log "!!! Set a password for 'prometheus' with: passwd prometheus"
fi

# 6. Configure sudo
log "Configuring sudo..."
echo "%wheel ALL=(ALL:ALL) ALL" > /etc/sudoers.d/wheel

# 7. Configure services
log "Enabling services..."
systemctl enable systemd-resolved
systemctl enable systemd-timesyncd
systemctl enable fwupd
systemctl enable bluetooth
systemctl enable pkgfile-update.timer
systemctl enable reflector.timer

# 8. Create Prometheus directories
log "Creating Prometheus directories..."
mkdir -p /etc/prometheus
mkdir -p /var/log/prometheus
mkdir -p /var/lib/prometheus/{memory,knowledge,models}
mkdir -p /usr/share/prometheus/{wallpapers,themes,icons}

# 9. Configure sysctl for performance
log "Configuring kernel parameters..."
cat > /etc/sysctl.d/99-prometheus.conf << 'SYSCTL'
# Prometheus OS Performance Tuning

# Memory
vm.swappiness=10
vm.vfs_cache_pressure=50
vm.dirty_ratio=10
vm.dirty_background_ratio=5
vm.max_map_count=1048576
vm.transparent_hugepages=always

# Network
net.core.rmem_max=134217728
net.core.wmem_max=134217728
net.ipv4.tcp_rmem=4096 87380 134217728
net.ipv4.tcp_wmem=4096 65536 134217728
net.core.netdev_max_backlog=5000
net.ipv4.tcp_congestion_control=bbr
net.core.default_qdisc=fq

# Kernel
kernel.nmi_watchdog=0
kernel.random.read_wakeup_threshold=64
kernel.random.write_wakeup_threshold=128
kernel.sched_autogroup_enabled=0
kernel.sched_migration_cost_ns=5000000
SYSCTL

sysctl --system

# 10. CPU governor
log "Setting performance governor..."
for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
    echo "performance" > "$cpu" 2>/dev/null || true
done

log "=== Bootstrap complete ==="
log "Reboot to start Prometheus OS!"
