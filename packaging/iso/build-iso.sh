#!/usr/bin/env bash
# Prometheus OS ISO Builder
# Builds a bootable Arch Linux ISO with Prometheus OS pre-installed
set -euo pipefail

ISO_NAME="prometheus-os-0.1.0-x86_64"
WORK_DIR="/tmp/prometheus-iso-build"
PROJECT_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "=== Prometheus OS ISO Builder ==="

if ! command -v mkarchiso &>/dev/null; then
    echo "Error: archiso not installed. Install with: pacman -S archiso"
    exit 1
fi

rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR"
cp -r /usr/share/archiso/configs/releng "$WORK_DIR/releng"
cd "$WORK_DIR/releng"

echo "prometheus" > efiboot/loader/entries/01-prometheus.conf
cat > efiboot/loader/entries/01-prometheus.conf << 'BOOTENTRY'
title   Prometheus OS
linux   /vmlinuz-linux-zen
initrd  /intel-ucode.img
initrd  /amd-ucode.img
initrd  /initramfs-linux-zen.img
options root=LABEL=PROMETHEUS_ROOT rw rootflags=subvol=@ quiet splash loglevel=3 rd.systemd.show_status=auto rd.udev.log_level=3 vt.global_cursor_default=0
BOOTENTRY

cat > efiboot/loader/entries/02-prometheus-recovery.conf << 'BOOTENTRY'
title   Prometheus OS (Recovery Mode)
linux   /vmlinuz-linux-zen
initrd  /intel-ucode.img
initrd  /amd-ucode.img
initrd  /initramfs-linux-zen.img
options root=LABEL=PROMETHEUS_ROOT rw rootflags=subvol=@ systemd.unit=rescue.target
BOOTENTRY

# Install base + Prometheus packages
cat >> packages.x86_64 << 'PACKAGES'
linux-zen linux-firmware amd-ucode intel-ucode
mesa mesa-utils vulkan-icd-loader vulkan-intel vulkan-radeon
pipewire pipewire-alsa pipewire-pulse wireplumber
wlroots wayland wayland-protocols libxkbcommon libinput
systemd btrfs-progs snapper snap-pac
sbctl apparmor bubblewrap
plymouth
networkmanager iwd bluez bluez-utils
inter-font noto-fonts noto-fonts-emoji ttf-jetbrains-mono
pacman-contrib reflector fwupd
python nodejs rust cargo
base-devel git cmake clang
flatpak
PACKAGES

# Prometheus OS branding overlay
mkdir -p airootfs/usr/share/pixmaps
cp "$PROJECT_ROOT/resources/brand/logo.svg" airootfs/usr/share/pixmaps/prometheus.svg 2>/dev/null || true

mkdir -p airootfs/usr/share/backgrounds/prometheus
cp "$PROJECT_ROOT/resources/wallpapers/prometheus-default.svg" airootfs/usr/share/backgrounds/prometheus/default.svg 2>/dev/null || true
cp "$PROJECT_ROOT/resources/wallpapers/prometheus-lock.svg" airootfs/usr/share/backgrounds/prometheus/lock.svg 2>/dev/null || true

mkdir -p airootfs/usr/share/plymouth/themes/prometheus
cp -r "$PROJECT_ROOT/resources/plymouth/"* airootfs/usr/share/plymouth/themes/prometheus/ 2>/dev/null || true

mkdir -p airootfs/etc
cat > airootfs/etc/hostname << 'HOSTNAME'
prometheus
HOSTNAME

cat > airootfs/etc/mkinitcpio.conf << 'MKINITCPIO'
MODULES=(btrfs intel_agp i915 amdgpu nvidia nvidia_modeset nvidia_uvm nvidia_drm)
BINARIES=(/usr/bin/btrfs)
FILES=()
HOOKS=(base udev autodetect modconf kms keyboard keymap consolefont block filesystems fsck)
MKINITCPIO

# os-release
cat > airootfs/etc/os-release << 'EOF'
NAME="Prometheus OS"
ID=prometheus
PRETTY_NAME="Prometheus OS 0.1 (Prometheus)"
VERSION_ID="0.1"
VERSION_CODENAME=prometheus
ID_LIKE=arch
HOME_URL="https://prometheus-os.dev"
LOGO=prometheus
ANSI_COLOR="0;34"
EOF

echo "Building ISO..."
ISO_OUTPUT="$(pwd)/${ISO_NAME}.iso"
mkarchiso -v -w "$WORK_DIR" -o "$(pwd)" .

if [ -f "${ISO_NAME}.iso" ]; then
    echo "=== ISO built successfully: ${ISO_NAME}.iso ==="
    echo "Size: $(du -h "${ISO_NAME}.iso" | cut -f1)"
    echo ""
    echo "To boot in QEMU:"
    echo "  qemu-system-x86_64 -enable-kvm -cdrom ${ISO_NAME}.iso -m 4096"
else
    echo "Error: ISO build failed"
    exit 1
fi
