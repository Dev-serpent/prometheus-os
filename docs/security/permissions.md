# Permission Model

Prometheus OS uses a capability-based permission system where no process or AI action has ambient authority. Every operation must be explicitly authorized.

## Permission Categories

| Category | Examples | Default |
|----------|----------|---------|
| `filesystem.read` | Read files and directories | Ask |
| `filesystem.write` | Create, modify, delete files | Ask |
| `network.http` | Make HTTP requests | Allow |
| `network.listen` | Listen on network ports | Deny |
| `system.command` | Execute shell commands | Deny |
| `system.process` | List/manage processes | Allow |
| `system.service` | Start/stop system services | Deny |
| `hardware.usb` | Access USB devices | Ask |
| `hardware.gpio` | Access GPIO pins | Deny |
| `hardware.bluetooth` | Bluetooth operations | Ask |
| `ai.memory.read` | Read AI memory graph | Allow |
| `ai.memory.write` | Write to AI memory graph | Allow |
| `ai.reasoning` | Intercept/modify reasoning | Deny |
| `user.data` | Access user profile data | Ask |
| `user.location` | Access location data | Ask |
| `user.notifications` | Send notifications | Allow |

## Permission Levels

| Level | Behavior |
|-------|----------|
| `Allow` | Auto-approved, logged |
| `Deny` | Auto-rejected, logged |
| `Ask` | Prompt user (with optional remember) |
| `AskOnce` | Prompt user, no persistence |
| `Temporary` | Allow for N minutes |

## Configuration

Permissions are stored in `/etc/prometheus/permissions.toml`:

```toml
[defaults]
filesystem.read = "ask"
network.http = "allow"
system.command = "deny"

[apps.firefox]
network.http = "allow"
filesystem.read = "allow"

[plugins.my-plugin]
"ai.memory.read" = "allow"
"network.http" = "allow"
```

## Audit

Every permission check is logged:

```
2026-07-23T14:30:00+00:00 | ALLOW | firefox | network.http | https://api.example.com
2026-07-23T14:30:01+00:00 | DENY  | unknown-plugin | system.command | "rm -rf /"
```
