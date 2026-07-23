# Architecture Overview

Prometheus OS is designed as a layered system where each layer has a single responsibility and communicates through well-defined interfaces. The AI Core sits at the center, mediating between user intent and system execution.

## System Layers

```mermaid
graph TB
    subgraph L0["Layer 0 — Hardware"]
        CPU[CPU / GPU]
        MEM[RAM]
        DISK[Storage]
        NET[Network]
        IO[Peripherals]
    end

    subgraph L1["Layer 1 — Kernel"]
        LIN[linux-zen]
        DRV[Drivers]
        FS[Btrfs]
        SEC_LSM[LSM / AppArmor]
    end

    subgraph L2["Layer 2 — System Services"]
        WAY[Wayland]
        PW[PipeWire]
        SYS[systemd]
        DBS[DBus]
    end

    subgraph L3["Layer 3 — Platform"]
        COMP[Compositor]
        DSK[Desktop Shell]
        SEC[Security Daemon]
        AI[AI Core]
        APP[Application Runtime]
    end

    subgraph L4["Layer 4 — Applications"]
        TERM[Terminal]
        FILE[Files]
        BRW[Browser]
        SET[Settings]
        DSH[Dashboard]
        PKG[Package Manager]
        MON[System Monitor]
        SNP[Snapshot Manager]
        PLG[Plugin Manager]
        DEV[Developer Hub]
    end

    subgraph L5["Layer 5 — User"]
        NL[Natural Language]
        VOICE[Voice]
        GEST[Gestures]
        VISION[Screen Vision]
    end

    L0 --> L1
    L1 --> L2
    L2 --> L3
    L3 --> L4
    L4 --> L5
    AI -.-> L5
    AI -.-> L4
    AI -.-> L2
```

## Design Philosophy

### 1. AI-First Architecture

Every component exposes a machine-readable interface. The AI Core can inspect, query, and control any subsystem. There are no "user-only" APIs — if you can click it, the AI can call it.

### 2. Latency Budget

| Operation | Budget | Measured |
|-----------|--------|----------|
| AI response | < 100 ms | ~45 ms |
| Frame render | < 4.2 ms | ~6.9 ms |
| Boot to desktop | < 5 s | ~8 s |
| Wake from suspend | < 500 ms | ~300 ms |
| App launch | < 200 ms | ~150 ms |

### 3. Zero-Cost Abstractions

Rust's ownership model enables safe systems programming without garbage collection overhead. The compositor renders directly to Vulkan with zero intermediate buffers.

### 4. Capability-Based Security

No process runs with ambient authority. Every AI action, every app launch, every system call is gated by explicit capability grants configured per-application and per-user.

## Core Communication Pathways

```mermaid
sequenceDiagram
    participant U as User
    participant AI as AI Core
    participant KG as Knowledge Graph
    participant COMP as Compositor
    participant APP as Application

    U->>AI: "Open my project files"
    AI->>KG: Query memory context
    KG-->>AI: Project location, recent files
    AI->>COMP: Focus workspace 2
    COMP-->>AI: Workspace activated
    AI->>APP: Launch file manager at path
    APP-->>AI: Window opened
    AI-->>U: "Your project files are ready"
```

## Data Flow

```mermaid
flowchart LR
    subgraph Input
        TXT[Text]
        SPC[Speech]
        CAP[Screen Capture]
    end
    subgraph Process
        INT[Intent Parser]
        CTX[Context Builder]
        RSN[Reasoning]
    end
    subgraph Act
        CMD[Command Exec]
        SYS[System Call]
        APP[App Launch]
    end
    subgraph Learn
        OBS[Observation]
        UPD[Memory Update]
        PAT[Pattern Detection]
    end

    Input --> Process
    Process --> Act
    Act --> Learn
    Learn -.-> Process
```

## Component Dependency Graph

```mermaid
graph LR
    subgraph W["Workspace"]
        AI[ai-core]
        COMP[compositor]
        DSK[desktop]
        SEC[security]
        SDK[sdk/rust]
        TERM[applications/terminal]
        FILES[applications/files]
        BRW[applications/browser]
        SET[applications/settings]
        DSH[applications/dashboard]
        PKG[applications/package-manager]
        MON[applications/system-monitor]
        SNP[applications/snapshot-manager]
        PLG[applications/plugin-manager]
        DEV[applications/developer-hub]
    end

    DSK --> COMP
    DSK --> AI
    TERM --> COMP
    DSH --> AI
    DSH --> MON
    SET --> AI
    SDK --> AI
    PLG --> AI
    DEV --> SDK
```

## Next Steps

- [System Design Deep Dive](system-design.md)
- [Kernel Integration](kernel.md)
- [Compositor Architecture](compositor.md)
- [AI Core Architecture](../ai-core/index.md)
