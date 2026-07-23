# AI Core Configuration

The AI Core is configured through `/etc/prometheus/ai.conf`. All configuration values have sensible defaults and can be overridden via environment variables.

## Reference

```toml
[core]
# Path to AI model storage
model_path = "/usr/lib/prometheus/models"
# Maximum context window tokens
context_window = 4096
# Maximum concurrent agent instances
max_agents = 4
# AI response timeout in milliseconds
timeout_ms = 10000
# Log level: trace, debug, info, warn, error
log_level = "info"

[memory]
# Knowledge graph storage path
graph_db_path = "/var/lib/prometheus/memory"
# Vector embedding dimension
vector_dimension = 768
# Automatically prune old memories
auto_prune = true
# Prune memories older than this many days
prune_threshold_days = 90
# Maximum memory nodes before pruning
max_nodes = 100000
# Memory persistence interval in seconds
persist_interval_sec = 30

[voice]
# Enable voice interaction
enabled = true
# Wake word phrase
wake_word = "prometheus"
# Audio input device (default: system default)
input_device = "default"
# Audio output device
output_device = "default"

[voice.wake_word]
# Wake word engine: porcupine, snowboy, custom
engine = "porcupine"
# Detection sensitivity 0.0-1.0
sensitivity = 0.7
# Confidence threshold 0.0-1.0
threshold = 0.5

[voice.stt]
# Speech-to-text model: whisper-base, whisper-small, whisper-medium
model = "whisper-base"
# Language code (empty = auto-detect)
language = "en"
# Beam search width
beam_size = 5
# Compute type: float32, float16, int8
compute_type = "int8"

[voice.tts]
# Text-to-speech engine: piper, espeak, custom
engine = "piper"
# Voice model name
voice = "en_US-amy-medium"
# Speech speed 0.5-2.0
speed = 1.0
# Speech pitch 0.5-2.0
pitch = 1.0

[vision]
# Enable screen vision
enabled = true
# Screen capture frame rate
capture_fps = 5
# OCR engine: tesseract, easyocr, custom
ocr_engine = "tesseract"
# Enable UI element detection
element_detection = true
# Maximum capture resolution
max_resolution = "1920x1080"

[reasoning]
# Maximum ReAct iterations
max_iterations = 10
# Minimum confidence for auto-response
confidence_threshold = 0.6
# Enable fallback to rule engine
enable_fallback = true
# Fallback model type
fallback_model = "rule-engine"
# Reasoning timeout in milliseconds
timeout_ms = 5000

[automation]
# Enable workflow learning
enabled = true
# Pattern learning rate 0.0-1.0
learning_rate = 0.1
# Minimum observations before suggesting
min_observations = 3
# Confidence threshold for auto-suggest
confidence_threshold = 0.7
# Auto-execute without asking
auto_execute = false
# Maximum suggestions per day
max_suggestions_per_day = 5
# Pattern time window in minutes
time_window_minutes = 5

[planner]
# Maximum concurrent tasks
max_concurrent_tasks = 4
# Default task timeout in seconds
default_timeout_sec = 300
# Maximum retries per task
max_retries = 3
# Enable auto-recovery on failure
enable_recovery = true
# Ask user when plan fails
ask_user_on_failure = true

[graph]
# Graph database path
base_path = "/var/lib/prometheus/graph"
# Maximum entity nodes
max_nodes = 100000
# Maximum relationship edges
max_edges = 500000
# Auto-save interval in seconds
auto_persist_interval_sec = 30
# Enable write-ahead log
enable_wal = true
# Distance metric: cosine, euclidean, dot
index_distance = "cosine"

[models]
# Default model for routing
default_model = "onnx"

[models.routing]
  [models.routing.simple_command]
  pattern = "^(open|launch|close|quit|run) "
  model = "onnx"
  max_tokens = 50

  [models.routing.reasoning]
  pattern = "(analyze|compare|explain|why|how|what if)"
  model = "llama"
  max_tokens = 512

  [models.routing.code]
  pattern = "(write|create|implement|refactor) (code|function|class|script)"
  model = "remote"
  max_tokens = 2048

[models.onnx]
path = "/usr/lib/prometheus/models/command.onnx"
execution_provider = "cpu"
intra_op_threads = 4
inter_op_threads = 2

[models.llama]
path = "/usr/lib/prometheus/models/llama-7b.gguf"
n_gpu_layers = 32
n_ctx = 4096
n_batch = 512
n_threads = 8
```

## Environment Variables

| Variable | Overrides | Default |
|----------|-----------|---------|
| `PROMETHEUS_AI_CONFIG` | Config file path | `/etc/prometheus/ai.conf` |
| `PROMETHEUS_MODEL_PATH` | `core.model_path` | `/usr/lib/prometheus/models` |
| `PROMETHEUS_LOG_LEVEL` | `core.log_level` | `info` |
| `PROMETHEUS_MEMORY_PATH` | `memory.graph_db_path` | `/var/lib/prometheus/memory` |

## Next Steps

- [AI Core Overview](index.md)
- [Model Integration](models.md)
- [Plugin System](plugins.md)
