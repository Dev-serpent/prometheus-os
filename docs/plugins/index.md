# Plugin System

The Prometheus OS plugin system enables third-party developers to extend AI capabilities, add new tools, and integrate external services.

## Overview

```mermaid
graph TB
    subgraph Host["Prometheus OS"]
        PM[Plugin Manager]
        REG[Plugin Registry]
        API[Plugin API]
        SEC[Security Sandbox]
    end
    subgraph Plugins["Installed Plugins"]
        P1[Web Search]
        P2[Calendar]
        P3[Code Analysis]
        P4[Weather]
        P5[Git Integration]
        PN[...more]
    end
    subgraph Sources
        MKT[Plugin Marketplace]
        LOCAL[Local Build]
        DEV[Development Mode]
    end

    Sources --> PM
    PM --> REG
    REG --> P1
    REG --> P2
    REG --> P3
    REG --> P4
    REG --> P5
    REG --> PN
    API --> P1
    API --> P2
    SEC --> P1
    SEC --> P2
```

## Capabilities

- **Hot-reload**: Install and update plugins without restarting
- **Sandboxed**: Each plugin runs in isolated bubblewrap container
- **Permission-based**: Granular capability control
- **Versioned**: API compatibility guarantees
- **Signed**: Cryptographic verification of plugin origin
- **Scoped**: Limited to declared capabilities

## Plugin Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Discovered
    Discovered --> Validated: Manifest check
    Validated --> Verified: Signature check
    Verified --> Installed: User confirms
    Installed --> Enabled: Permissions granted
    Enabled --> Running: Hook registration
    Running --> Disabled: User disables
    Disabled --> Enabled: Re-enabled
    Disabled --> Uninstalled: Removed
    Running --> Error: Exception
    Error --> Disabled: Recovery
```

## Quick Start

Create a simple plugin:

```rust
use prometheus_sdk::plugin::*;

#[plugin]
pub struct GreeterPlugin;

#[plugin_hook]
impl PluginHooks for GreeterPlugin {
    async fn on_register(&self, ctx: &PluginContext) -> Result<()> {
        ctx.register_command("hello", |args| {
            let name = args.get("name").unwrap_or("World");
            format!("Hello, {}! Welcome to Prometheus OS.", name)
        });
        Ok(())
    }
}
```

## Next Steps

- [Plugin Architecture](architecture.md) — Deep dive into design
- [Development Guide](development.md) — Building your first plugin
- [Plugin API](api.md) — Complete API reference
- [Plugin Manifest](manifest.md) — Manifest format reference
- [Permissions](permissions.md) — Security model
- [Marketplace](marketplace.md) — Publishing and distribution
