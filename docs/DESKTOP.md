# Prometheus Desktop Environment

## Design Philosophy

Prometheus Desktop is not another Linux desktop environment. It is an AI-native interface that reimagines human-computer interaction. The traditional desktop metaphor is preserved where useful but enhanced with intelligence throughout.

## Visual Design

### Color Scheme
- **Background**: `#0A0A0A` (near black)
- **Surface**: `#1A1A1A` (dark grey)
- **Text Primary**: `#FFFFFF` (white)
- **Text Secondary**: `#A0A0A0` (grey)
- **Accent**: `#0078FF` (electric blue)
- **Glow**: `#0078FF` (soft blue glow)
- **Success**: `#00C853` (green)
- **Warning**: `#FFD600` (amber)
- **Error**: `#FF1744` (red)

### Design Elements
- **Glassmorphism**: Translucent panels with backdrop blur
- **Minimalism**: Clean, uncluttered interfaces
- **Depth**: Subtle shadows and elevation
- **Motion**: Physics-based animations
- **Typography**: Inter font family
- **Rounding**: 12px corner radius on surfaces

## Compositor Features

### Window Management
- **Dynamic Tiling**: Auto-arranges windows with master-stack layout
- **Floating Mode**: Traditional free-form window placement
- **Hybrid**: Per-window tiling/floating choice
- **Monocle**: Full-screen focus mode
- **Grid**: Equal-sized grid layout

### Animations
- **Physics Engine**: Spring-based animations using Hooke's law
- **Window Open/Close**: Scale + fade with spring curve
- **Workspace Switch**: Slide with easing
- **Minimize/Unminimize**: Physics-based transition
- **Focus Change**: Smooth border highlight

### Effects
- **Blur**: GPU-accelerated Kawase blur on panels and backgrounds
- **Shadows**: Dynamic drop shadows with variable spread
- **Glow**: Accent-colored glow on focused elements
- **Corners**: Hardware-accelerated rounded corners

### Performance
- **Target**: 60-240 FPS
- **Direct Scanout**: Zero-copy buffer submission
- **Adaptive Sync**: Variable refresh rate support
- **Tear-Free**: Atomic modesetting
- **Idle Inhibit**: Prevents sleep during fullscreen

## Panel

- 44px height
- Translucent with blur
- Left: App Launcher, Workspace indicators
- Center: Task bar with window previews
- Right: AI status, Notifications, Clock, System tray, Quick settings
- Auto-hide option

## Launcher

- Activated by clicking logo or Super+Space
- AI-powered search across apps, files, settings, web
- Recent and suggested applications
- Inline calculator, terminal, and web search
- Dark, full-screen overlay with blur background

## Workspaces

- 9 workspaces by default
- Visual workspace switcher with previews
- Drag windows between workspaces
- Per-workspace wallpaper support
- Animation on switch
