# Using the AI

The AI is your primary interface to Prometheus OS. It understands natural language, maintains context across sessions, and learns your workflow.

## Invoking the AI

### Voice Activation

Say **"Hey Prometheus"** followed by your command. The wake word is configurable in settings.

### Keyboard Shortcut

Press ++super+space++ to open the AI query prompt. Type your command and press ++enter++.

### Terminal

```bash
prometheus-ai --query "your command here"
prometheus-ai --interactive  # Chat mode
```

## What You Can Ask

### System Control

```
"Open terminal"
"Switch to workspace 3"
"Close all windows"
"Turn up the volume"
"Connect to Office WiFi"
"Enable dark mode"
```

### Information Queries

```
"What's my CPU usage?"
"How much RAM is free?"
"Show me my top processes"
"What's the weather?"
"What time is it in Tokyo?"
"Who sent me emails today?"
```

### File Operations

```
"Find my project files"
"Show me recent downloads"
"Move the report to the project folder"
"Create a new document called meeting-notes"
"Compress the photo album"
```

### Development

```
"Initialize a new Rust project"
"Run the tests"
"Show me git status"
"Deploy the web app"
"Find the bug in this code"
```

### Automation

```
"Every morning at 9 AM, open my calendar and email"
"When I connect to the office network, start Slack"
"After compiling, run the tests"
"Remind me to submit the report at 5 PM"
```

## Context Awareness

The AI remembers:
- What you're currently working on
- Recently opened files and applications
- Your preferences and habits
- The current workspace layout
- Active conversations

You can refer to context naturally:

```
"Open that file I was editing earlier"
"Send it to John" (knows what "it" refers to)
"Continue where I left off"
```

## Voice vs. Text

Prometheus OS supports both equally. Switch seamlessly:

- **Voice**: Natural for quick commands and queries
- **Text**: Better for complex, multi-step instructions
- **Mixed**: Start with voice, refine with text

## Configuration

The AI can be configured in Settings → AI:

- Wake word sensitivity
- Voice speed and pitch
- Model selection (local vs. cloud)
- Privacy controls
- Learning preferences
