# Automation Engine

The Automation Engine observes user behavior over time, detects recurring patterns, and autonomously executes learned workflows.

## Pattern Detection Pipeline

```mermaid
graph LR
    subgraph Observe
        EVT[Event Stream]
        BUF[Event Buffer]
        SEQ[Sequence Builder]
    end
    subgraph Analyze
        PAT[Pattern Matcher]
        FREQ[Frequency Analysis]
        CORR[Correlation]
    end
    subgraph Act
        SUG[Suggest Automation]
        AUT[Auto-Execute]
        LRN[Update Model]
    end

    EVT --> BUF
    BUF --> SEQ
    SEQ --> PAT
    PAT --> FREQ
    FREQ --> CORR
    CORR --> SUG
    CORR --> AUT
    AUT --> LRN
```

## Event Types

The engine tracks these user events:

| Event | Trigger | Example |
|-------|---------|---------|
| `AppLaunched` | Application started | `Firefox` |
| `FileOpened` | File accessed | `/docs/report.md` |
| `CommandRun` | Terminal command | `git push` |
| `WindowFocus` | Window changed | `Code - main.rs` |
| `WorkspaceSwitch` | Workspace changed | Workspace 3 |
| `NetworkJoin` | WiFi connected | `Office-Network` |
| `DevicePlugged` | USB/HDMI connected | External monitor |

## Pattern Types

### Sequential Patterns

```json
{
  "type": "sequential",
  "events": [
    "AppLaunched: Terminal",
    "CommandRun: cd projects/my-app",
    "CommandRun: npm run dev"
  ],
  "frequency": 15,
  "time_window_minutes": 2,
  "confidence": 0.92
}
```

### Conditional Patterns

```json
{
  "type": "conditional",
  "trigger": "NetworkJoin: Office-Network",
  "then": [
    "AppLaunched: Slack",
    "AppLaunched: Firefox",
    "SetWorkspace: 1: Communication"
  ],
  "invocations": 87,
  "confidence": 0.98
}
```

### Temporal Patterns

```json
{
  "type": "temporal",
  "schedule": "weekday 09:00",
  "actions": [
    "OpenCalendar",
    "ShowWeather",
    "StartFocusMode"
  ],
  "invocations": 42,
  "confidence": 0.85
}
```

## Automation Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Observing: System starts
    Observing --> PatternDetected: 3+ occurrences
    PatternDetected --> Suggesting: Confidence > 0.7
    Suggesting --> Confirmed: User accepts
    Suggesting --> Discarded: User dismisses
    Confirmed --> Active: Schedule set
    Active --> Executing: Trigger matches
    Executing --> Active: Completed
    Executing --> Failed: Error
    Failed --> Observing: Pattern broken
    Discarded --> Observing
```

## Suggested Automation UI

When the engine detects a pattern, it surfaces a suggestion:

```
┌─────────────────────────────────────────┐
│  🔄 Automation Detected                  │
│                                         │
│  I notice you always open Slack,        │
│  Firefox, and VS Code when you join     │
│  your office network.                   │
│                                         │
│  Auto-run this when connecting to       │
│  "Office-Network"?                      │
│                                         │
│  [Yes, Save] [Run Once] [Not Now]       │
└─────────────────────────────────────────┘
```

## Configuration

```toml
[automation]
enabled = true
learning_rate = 0.1
min_observations = 3
confidence_threshold = 0.7
auto_execute = false
max_suggestions_per_day = 5
time_window_minutes = 5
event_buffer_size = 10000
```

## Next Steps

- [Planner](planner.md) — Multi-step task decomposition
- [Knowledge Graph](graph.md) — Entity relationships
- [Reasoning Engine](reasoning.md) — Core logic processing
