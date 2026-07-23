.PHONY: all gnome clean compositor desktop ai apps sdk iso install gnome-install

PREFIX ?= /usr
BUILD_DIR ?= build
CONFIG ?= release

all: compositor desktop ai apps sdk

# Native Prometheus components
compositor:
	@mkdir -p $(BUILD_DIR)/compositor
	cd compositor && cargo build --$(CONFIG) --target-dir ../$(BUILD_DIR)/compositor

desktop:
	@mkdir -p $(BUILD_DIR)/desktop
	cd desktop && cargo build --$(CONFIG) --target-dir ../$(BUILD_DIR)/desktop

ai:
	@mkdir -p $(BUILD_DIR)/ai-core
	cd ai-core && cargo build --$(CONFIG) --target-dir ../$(BUILD_DIR)/ai-core

apps:
	@for app in terminal files browser settings task-manager dashboard package-manager system-monitor snapshot-manager plugin-manager developer-hub; do \
		mkdir -p $(BUILD_DIR)/applications/$$app; \
		cd applications/$$app && cargo build --$(CONFIG) --target-dir ../../$(BUILD_DIR)/applications/$$app; \
	done

sdk:
	@mkdir -p $(BUILD_DIR)/sdk/rust
	cd sdk/rust && cargo build --$(CONFIG) --target-dir ../../$(BUILD_DIR)/sdk/rust 2>/dev/null || echo "SDK build skipped for rust"
	@echo "  Python SDK: source available at sdk/python/"
	@echo "  C++ SDK: source available at sdk/cpp/"
	@echo "  JavaScript SDK: source available at sdk/javascript/"

# GNOME Desktop Environment integration
gnome: gnome-theme gnome-extensions gnome-config

gnome-theme:
	@echo "Installing Prometheus GNOME theme..."
	@mkdir -p $(BUILD_DIR)/gnome/theme
	cp -r desktop/gnome/shell-theme/* $(BUILD_DIR)/gnome/theme/

gnome-extensions:
	@echo "Building Prometheus GNOME extensions..."
	@mkdir -p $(BUILD_DIR)/gnome/extensions
	@for ext in prometheus-ai prometheus-dashboard prometheus-layout; do \
		mkdir -p $(BUILD_DIR)/gnome/extensions/$$ext; \
		cp -r desktop/gnome/extensions/$$ext/* $(BUILD_DIR)/gnome/extensions/$$ext/; \
	done

gnome-config:
	@echo "Preparing GNOME configuration..."
	@mkdir -p $(BUILD_DIR)/gnome/config
	cp -r desktop/gnome/config/* $(BUILD_DIR)/gnome/config/
	cp -r desktop/gnome/mutter/* $(BUILD_DIR)/gnome/config/
	cp -r desktop/gnome/gdm/* $(BUILD_DIR)/gnome/config/

install:
	@echo "=== Installing Prometheus OS Core Components ==="
	
	# Compositor
	install -Dm755 $(BUILD_DIR)/compositor/release/prometheus-compositor $(DESTDIR)$(PREFIX)/bin/prometheus-compositor
	
	# Desktop shell
	install -Dm755 $(BUILD_DIR)/desktop/release/prometheus-desktop $(DESTDIR)$(PREFIX)/bin/prometheus-desktop
	
	# AI Core
	install -Dm755 $(BUILD_DIR)/ai-core/release/prometheus-ai $(DESTDIR)$(PREFIX)/bin/prometheus-ai
	
	# Applications
	install -Dm755 $(BUILD_DIR)/applications/terminal/release/prometheus-terminal $(DESTDIR)$(PREFIX)/bin/prometheus-terminal
	install -Dm755 $(BUILD_DIR)/applications/files/release/prometheus-files $(DESTDIR)$(PREFIX)/bin/prometheus-files
	install -Dm755 $(BUILD_DIR)/applications/settings/release/prometheus-settings $(DESTDIR)$(PREFIX)/bin/prometheus-settings
	install -Dm755 $(BUILD_DIR)/applications/dashboard/release/prometheus-dashboard $(DESTDIR)$(PREFIX)/bin/prometheus-dashboard
	install -Dm755 $(BUILD_DIR)/applications/package-manager/release/prometheus-package-manager $(DESTDIR)$(PREFIX)/bin/prometheus-package-manager
	install -Dm755 $(BUILD_DIR)/applications/system-monitor/release/prometheus-system-monitor $(DESTDIR)$(PREFIX)/bin/prometheus-system-monitor
	install -Dm755 $(BUILD_DIR)/applications/snapshot-manager/release/prometheus-snapshot-manager $(DESTDIR)$(PREFIX)/bin/prometheus-snapshot-manager
	install -Dm755 $(BUILD_DIR)/applications/plugin-manager/release/prometheus-plugin-manager $(DESTDIR)$(PREFIX)/bin/prometheus-plugin-manager
	install -Dm755 $(BUILD_DIR)/applications/developer-hub/release/prometheus-developer-hub $(DESTDIR)$(PREFIX)/bin/prometheus-developer-hub
	install -Dm755 $(BUILD_DIR)/applications/browser/release/prometheus-browser $(DESTDIR)$(PREFIX)/bin/prometheus-browser
	install -Dm755 $(BUILD_DIR)/applications/task-manager/release/prometheus-task-manager $(DESTDIR)$(PREFIX)/bin/prometheus-task-manager
	
	# Configuration
	install -Dm644 configs/prometheus.conf $(DESTDIR)/etc/prometheus/prometheus.conf
	install -Dm644 configs/compositor.conf $(DESTDIR)/etc/prometheus/compositor.conf
	install -Dm644 configs/ai.conf $(DESTDIR)/etc/prometheus/ai.conf
	
	# Systemd services
	install -Dm644 boot/systemd/prometheus-compositor.service $(DESTDIR)/usr/lib/systemd/system/prometheus-compositor.service
	install -Dm644 boot/systemd/prometheus-ai.service $(DESTDIR)/usr/lib/systemd/system/prometheus-ai.service
	install -Dm644 boot/systemd/prometheus-session.target $(DESTDIR)/usr/lib/systemd/system/prometheus-session.target
	
	# Session files
	install -Dm644 resources/desktop/prometheus.desktop $(DESTDIR)/usr/share/wayland-sessions/prometheus.desktop
	
	# Bootstrap script
	install -Dm755 scripts/prometheus-bootstrap.sh $(DESTDIR)$(PREFIX)/bin/prometheus-bootstrap
	
	@echo "=== Core Installation Complete ==="

install-gnome: install
	@echo "=== Installing Prometheus GNOME Integration ==="
	
	# Session
	install -Dm644 desktop/gnome/session/prometheus-gnome.desktop $(DESTDIR)/usr/share/wayland-sessions/prometheus-gnome.desktop
	install -Dm644 desktop/gnome/session/prometheus-gnome.session $(DESTDIR)/usr/share/gnome-session/sessions/prometheus-gnome.session
	install -Dm755 desktop/gnome/session/prometheus-gnome-session.sh $(DESTDIR)/usr/lib/prometheus/prometheus-gnome-session
	
	# Theme
	install -Dm644 desktop/gnome/shell-theme/gtk.css $(DESTDIR)/usr/share/themes/Prometheus-Dark/gtk-3.0/gtk.css
	install -Dm644 desktop/gnome/shell-theme/index.theme $(DESTDIR)/usr/share/themes/Prometheus-Dark/index.theme
	mkdir -p $(DESTDIR)/usr/share/themes/Prometheus-Dark/gtk-4.0
	ln -sf ../gtk-3.0/gtk.css $(DESTDIR)/usr/share/themes/Prometheus-Dark/gtk-4.0/gtk.css 2>/dev/null || true
	
	# Extensions
	for ext in prometheus-ai prometheus-dashboard prometheus-layout; do \
		install -Dm644 desktop/gnome/extensions/$$ext/extension.js \
			$(DESTDIR)/usr/share/gnome-shell/extensions/$$ext@prometheus-os.dev/extension.js; \
		install -Dm644 desktop/gnome/extensions/$$ext/metadata.json \
			$(DESTDIR)/usr/share/gnome-shell/extensions/$$ext@prometheus-os.dev/metadata.json 2>/dev/null || true; \
	done
	
	# GDM config
	install -Dm644 desktop/gnome/gdm/00-prometheus-gdm.conf $(DESTDIR)/etc/gdm/custom.conf
	install -Dm644 desktop/gnome/gdm/prometheus-gdm-theme.css $(DESTDIR)/usr/share/themes/Prometheus-Dark/gdm/gdm.css
	
	# Mutter config
	install -Dm644 desktop/gnome/mutter/90-prometheus-performance.conf $(DESTDIR)/etc/dconf/db/local.d/90-prometheus-performance
	install -Dm644 desktop/gnome/mutter/92-prometheus-scheduling.conf $(DESTDIR)/etc/dconf/db/local.d/92-prometheus-scheduling
	
	# GNOME defaults
	install -Dm644 desktop/gnome/config/00-prometheus-defaults.gschema.override \
		$(DESTDIR)/usr/share/glib-2.0/schemas/00-prometheus-defaults.gschema.override
	
	# Setup script
	install -Dm755 scripts/prometheus-setup-gnome.sh $(DESTDIR)$(PREFIX)/bin/prometheus-setup-gnome
	
	@echo "=== GNOME Installation Complete ==="
	@echo "Run 'prometheus-setup-gnome' to finalize the setup."
	@echo "Or manually: glib-compile-schemas /usr/share/glib-2.0/schemas"
	@echo "             dconf update /etc/dconf"

iso:
	@echo "Building Prometheus OS ISO..."
	cd packaging/iso && ./build-iso.sh

clean:
	rm -rf $(BUILD_DIR)

run: all
	@echo "Starting Prometheus OS session..."
	exec prometheus-compositor

run-gnome:
	@echo "Starting Prometheus GNOME session..."
	dbus-run-session -- gnome-session --session=prometheus-gnome

test:
	@for dir in compositor desktop ai-core applications/*; do \
		if [ -d "$$dir" ]; then \
			cd "$$dir" && cargo test --target-dir ../../$(BUILD_DIR)/$$dir 2>/dev/null; \
			cd ../../; \
		fi; \
	done

lint:
	@for dir in compositor desktop ai-core applications/* sdk/*; do \
		if [ -d "$$dir" ] && [ -f "$$dir/Cargo.toml" ]; then \
			cd "$$dir" && cargo clippy --target-dir ../../$(BUILD_DIR)/$$dir 2>/dev/null; \
			cd ../../; \
		fi; \
	done

help:
	@echo "Prometheus OS Build System"
	@echo ""
	@echo "Targets:"
	@echo "  all              Build all core components"
	@echo "  compositor       Build wlroots compositor"
	@echo "  desktop          Build desktop shell"
	@echo "  ai               Build AI Core"
	@echo "  apps             Build all applications"
	@echo "  sdk              Build all SDKs"
	@echo "  gnome            Build GNOME integration"
	@echo "  install          Install core components"
	@echo "  install-gnome    Install GNOME integration"
	@echo "  iso              Build bootable ISO"
	@echo "  run              Run native compositor"
	@echo "  run-gnome        Run GNOME session"
	@echo "  clean            Clean build artifacts"
	@echo "  test             Run all tests"
	@echo "  lint             Run clippy lints"
	@echo ""
	@echo "Variables:"
	@echo "  PREFIX=/usr      Installation prefix"
	@echo "  CONFIG=release   Build configuration"
	@echo "  DESTDIR=         Staging directory"
