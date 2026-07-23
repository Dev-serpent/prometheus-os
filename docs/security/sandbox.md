# Sandboxing

Every application and plugin in Prometheus OS runs in an isolated sandbox using bubblewrap and Landlock LSM.

## Architecture

```mermaid
graph TB
    subgraph Host
        SB[Sandbox Manager]
        BP[Policy Engine]
    end
    subgraph Sandbox
        APP[Application]
        FS[/usr:/ro, /home/app-xdg:/rw]
        NET[Network Filter]
        SYS[Seccomp Filter]
        DEV[Device Whitelist]
    end
    subgraph Host_Resources
        REAL_FS[Full Filesystem]
        REAL_NET[Full Network]
        REAL_DEV[All Devices]
    end

    SB --> APP
    FS --x REAL_FS
    NET --x REAL_NET
    SYS --x REAL_DEV
```

## Default Restrictions

| Resource | Default | Rationale |
|----------|---------|-----------|
| Filesystem | Read: `/usr`, `/etc`; Write: app-specific XDG | Prevent tampering |
| Network | HTTP/HTTPS outbound only | No inbound by default |
| Devices | None unless granted | Hardware isolation |
| Processes | Own PID namespace | No process snooping |
| System calls | Seccomp allowlist | Kernel attack surface reduction |
| IPC | Own user namespace | No DBus without permission |

## Configuration

```rust
pub struct SandboxConfig {
    pub directories: Vec<DirMapping>,    // Host -> Sandbox mounts
    pub network: NetworkPolicy,          // Allowed protocols
    pub devices: Vec<DeviceAccess>,      // Allowed devices
    pub syscalls: SyscallPolicy,         // Seccomp filter
    pub capabilities: Vec<Capability>,   // Linux capabilities
    pub timeout: Duration,               // Max execution time
    pub memory_limit: Option<u64>,       // Max memory in bytes
}
```
