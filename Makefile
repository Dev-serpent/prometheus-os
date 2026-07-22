.PHONY: all clean compositor desktop ai apps iso install

PREFIX ?= /usr
BUILD_DIR ?= build
CONFIG ?= release

all: compositor desktop ai apps

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
	@for lang in python rust cpp javascript; do \
		cd sdk/$$lang && cargo build --$(CONFIG) --target-dir ../../$(BUILD_DIR)/sdk/$$lang; \
	done

install:
	@echo "Installing Prometheus OS components..."
	# Compositor
	install -Dm755 $(BUILD_DIR)/compositor/release/prometheus-compositor $(PREFIX)/bin/prometheus-compositor
	# Desktop shell
	install -Dm755 $(BUILD_DIR)/desktop/release/prometheus-shell $(PREFIX)/bin/prometheus-shell
	# AI Core
	install -Dm755 $(BUILD_DIR)/ai-core/release/prometheus-ai $(PREFIX)/bin/prometheus-ai
	install -Dm755 $(BUILD_DIR)/ai-core/release/prometheus-vision $(PREFIX)/bin/prometheus-vision
	install -Dm755 $(BUILD_DIR)/ai-core/release/prometheus-voice $(PREFIX)/bin/prometheus-voice
	install -Dm755 $(BUILD_DIR)/ai-core/release/prometheus-memory $(PREFIX)/bin/prometheus-memory
	install -Dm755 $(BUILD_DIR)/ai-core/release/prometheus-automation $(PREFIX)/bin/prometheus-automation
	# Applications
	install -Dm755 $(BUILD_DIR)/applications/terminal/release/prometheus-terminal $(PREFIX)/bin/prometheus-terminal
	install -Dm755 $(BUILD_DIR)/applications/files/release/prometheus-files $(PREFIX)/bin/prometheus-files
	install -Dm755 $(BUILD_DIR)/applications/browser/release/prometheus-browser $(PREFIX)/bin/prometheus-browser
	install -Dm755 $(BUILD_DIR)/applications/settings/release/prometheus-settings $(PREFIX)/bin/prometheus-settings
	install -Dm755 $(BUILD_DIR)/applications/dashboard/release/prometheus-dashboard $(PREFIX)/bin/prometheus-dashboard
	install -Dm755 $(BUILD_DIR)/applications/package-manager/release/prometheus-pkg $(PREFIX)/bin/prometheus-pkg
	install -Dm755 $(BUILD_DIR)/applications/system-monitor/release/prometheus-monitor $(PREFIX)/bin/prometheus-monitor
	# Configs
	install -Dm644 configs/prometheus.conf /etc/prometheus/prometheus.conf
	install -Dm644 configs/compositor.conf /etc/prometheus/compositor.conf
	install -Dm644 configs/ai.conf /etc/prometheus/ai.conf
	# Systemd services
	install -Dm644 boot/systemd/prometheus-compositor.service /usr/lib/systemd/system/prometheus-compositor.service
	install -Dm644 boot/systemd/prometheus-ai.service /usr/lib/systemd/system/prometheus-ai.service
	install -Dm644 boot/systemd/prometheus-session.target /usr/lib/systemd/system/prometheus-session.target
	# Desktop entries
	install -Dm644 resources/desktop/prometheus.desktop /usr/share/wayland-sessions/prometheus.desktop
	# Icons & themes
	cp -r resources/icons/* /usr/share/icons/
	cp -r resources/themes/* /usr/share/themes/

iso:
	@echo "Building Prometheus OS ISO..."
	cd packaging/iso && ./build-iso.sh

clean:
	rm -rf $(BUILD_DIR)

run: all
	@echo "Starting Prometheus OS session..."
	exec prometheus-compositor

test:
	@for dir in compositor desktop ai-core applications/*; do \
		cd $$dir && cargo test --target-dir ../../$(BUILD_DIR)/$$dir && cd ../../; \
	done

lint:
	@for dir in compositor desktop ai-core applications/*; do \
		cd $$dir && cargo clippy --target-dir ../../$(BUILD_DIR)/$$dir && cd ../../; \
	done
