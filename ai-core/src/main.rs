mod engine;
mod memory;
mod vision;
mod voice;
mod automation;
mod context;
mod plugin;
mod reasoning;

use engine::AIEngine;
use memory::MemoryGraph;
use vision::VisionEngine;
use voice::VoiceEngine;
use automation::AutomationEngine;
use context::ContextManager;
use reasoning::ReasoningEngine;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("prometheus_ai=info")
        .init();

    tracing::info!("Prometheus AI Core v{}", env!("CARGO_PKG_VERSION"));
    tracing::info!("Initializing AI subsystems...");

    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        let memory = MemoryGraph::new();
        let context = ContextManager::new(memory.clone());
        let reasoning = ReasoningEngine::new(memory.clone());
        let vision = VisionEngine::new();
        let voice = VoiceEngine::new();
        let automation = AutomationEngine::new(memory.clone());

        let engine = AIEngine::new(
            memory,
            context,
            reasoning,
            vision,
            voice,
            automation,
        );

        engine.run().await
    })
}
