# Compositor Architecture

The Prometheus compositor is a custom wlroots-based Wayland compositor designed for high-performance GPU-accelerated rendering with AI integration.

## Pipeline

```mermaid
graph TB
    subgraph Input
        KB[Keyboard]
        MS[Mouse]
        TP[Touchpad]
        TCH[Touchscreen]
    end
    subgraph Compositor
        IN[Input Manager]
        LYT[Layout Engine]
        REN[Vulkan Renderer]
        FX[Effects Pipeline]
        WSM[Workspace Manager]
    end
    subgraph Output
        MON[Monitor]
        VRR[Variable Refresh]
    end

    KB --> IN
    MS --> IN
    TP --> IN
    TCH --> IN
    IN --> LYT
    LYT --> REN
    REN --> FX
    FX --> MON
    FX --> VRR
    WSM --> LYT
```

## Render Loop

```mermaid
sequenceDiagram
    participant IN as Input
    participant LYT as Layout
    participant REN as Renderer
    participant GPU as GPU
    participant MON as Monitor

    loop Every Frame
        IN->>LYT: Input events
        LYT->>REN: Updated surface tree
        REN->>GPU: Begin frame
        REN->>GPU: Draw backgrounds
        REN->>REN: Apply effects (blur/glow/shadow)
        REN->>GPU: Draw surfaces
        REN->>GPU: Draw panel
        GPU-->>REN: Frame ready
        REN->>MON: Present (w/ VRR)
    end
```

## Frame Budget Analysis

| Phase | Budget (240 FPS) | Current |
|-------|-----------------|---------|
| Input processing | 0.5 ms | 0.3 ms |
| Layout computation | 0.5 ms | 0.4 ms |
| Vulkan command buffer | 1.0 ms | 1.2 ms |
| GPU rasterization | 1.0 ms | 3.0 ms |
| Effects (blur/glow) | 0.5 ms | 1.5 ms |
| Presentation | 0.67 ms | 0.5 ms |
| **Total** | **4.17 ms** | **6.9 ms** |
| **FPS** | **240** | **144** |

## Effects Pipeline

```mermaid
graph LR
    S[Surface] --> BL[Gaussian Blur]
    S --> GL[Glow Effect]
    S --> SH[Drop Shadow]
    BL --> CP[Composite]
    GL --> CP
    SH --> CP
    CP --> OUT[Output]
```

## Workspace Management

9 virtual workspaces with dynamic tiling layouts:

| Layout | Description |
|--------|-------------|
| **Master-Stack** | Primary window on left, stack on right |
| **Grid** | Equal-sized grid arrangement |
| **Floating** | Free-form window placement |
| **Monocle** | Full-screen focused window |
| **Dynamic** | AI-recommended layout based on context |

## Vulkan Rendering

```rust
pub struct VulkanRenderer {
    device: Arc<Device>,
    queue: Queue,
    swapchain: SwapchainKHR,
    render_pass: RenderPass,
    pipeline: Pipeline,
    descriptor_set: DescriptorSet,
    command_pool: CommandPool,
    command_buffers: Vec<CommandBuffer>,
    framebuffers: Vec<Framebuffer>,
    sync_objects: SyncObjects,
}
```

## Next Steps

- [Architecture Overview](index.md)
- [Desktop Environment](../desktop.md)
- [Performance Guide](../performance/index.md)
