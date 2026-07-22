# Prometheus Rust SDK

Build native AI-powered applications for Prometheus OS.

```toml
[dependencies]
prometheus-sdk = "0.1"
```

## Usage

```rust
use prometheus_sdk::PrometheusSDK;

fn main() {
    let sdk = PrometheusSDK::new();

    // Query the AI
    let response = sdk.ai().query("What's on my screen?");

    // Control the desktop
    sdk.desktop().send_notification("Hello", "From Prometheus!");

    // Access system resources
    let cpu = sdk.system().cpu_info();
}
```
