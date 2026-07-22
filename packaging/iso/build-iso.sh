#!/usr/bin/env bash
set -euo pipefail

# Prometheus OS ISO Builder
# Builds a bootable Arch Linux ISO with Prometheus OS pre-installed

ISO_NAME="prometheus-os-0.1.0-x86_64"
WORK_DIR="/tmp/prometheus-iso-build"
ISO_OUTPUT="../../${ISO_NAME}.iso"

echo "=== Prometheus OS ISO Builder ==="

# Prerequisites
if ! command -v mkarchiso &>/dev/null; then
    echo "Error: archiso not installed. Install with: pacman -S archiso"
    exit 1
fi

# Create working directory
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

# Create releng profile
if [ -d "releng" ]; then
    rm -rf releng
fi
cp -r /usr/share/archiso/configs/releng .

# Customize for Prometheus OS
cd releng

# Set hostname
echo "prometheus" > efiboot/loader/entries/01-prometheus.conf

# Configure boot loader entries
cat > efiboot/loader/entries/01-prometheus.conf << 'BOOTENTRY'
title   Prometheus OS
linux   /vmlinuz-linux-zen
initrd  /intel-ucode.img
initrd  /amd-ucode.img
initrd  /initramfs-linux-zen.img
options root=LABEL=PROMETHEUS_ROOT rw rootflags=subvol=@ quiet splash loglevel=3 rd.systemd.show_status=auto rd.udev.log_level=3 vt.global_cursor_default=0 mitigations=off nowatchdog nmi_watchdog=0
BOOTENTRY

cat > efiboot/loader/entries/02-prometheus-recovery.conf << 'BOOTENTRY'
title   Prometheus OS (Recovery Mode)
linux   /vmlinuz-linux-zen
initrd  /intel-ucode.img
initrd  /amd-ucode.img
initrd  /initramfs-linux-zen.img
options root=LABEL=PROMETHEUS_ROOT rw rootflags=subvol=@ mitigations=off systemd.unit=rescue.target
BOOTENTRY

# Configure mkarchiso
cat >> mkarchiso.conf << 'CONFIG'
# Prometheus OS build configuration
install_dir="prometheus"
arch="x86_64"
CONFIG

# Add Prometheus packages to pacman.conf
cat >> pacman.conf << 'PACMAN'
[prometheus]
SigLevel = Optional TrustAll
Server = https://repo.prometheus-os.dev/$arch
PACMAN

# Install Prometheus packages
echo "prometheus-meta" >> packages.x86_64
echo "prometheus-compositor" >> packages.x86_64
echo "prometheus-desktop" >> packages.x86_64
echo "prometheus-ai" >> packages.x86_64
echo "prometheus-apps" >> packages.x86_64

# Custom overlay
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

# Build the ISO
echo "Building ISO..."
mkarchiso -v -w "$WORK_DIR" -o "$WORK_DIR" .

# Copy ISO to output
if [ -f "${ISO_NAME}.iso" ]; then
    cp "${ISO_NAME}.iso" "$ISO_OUTPUT"
    echo "=== ISO built successfully: ${ISO_OUTPUT} ==="
    echo "Size: $(du -h "${ISO_OUTPUT}" | cut -f1)"
else
    echo "Error: ISO build failed"
    exit 1
fi
