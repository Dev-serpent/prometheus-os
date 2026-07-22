use crate::monitor::SystemMonitor;

pub struct DashboardRenderer;

impl DashboardRenderer {
    pub fn new() -> Self { Self }

    pub fn display(&self, m: &SystemMonitor) {
        print!("\x1b[2J\x1b[H");

        println!("\x1b[38;2;0;120;255m┌─────────────────────────────────────────────────────────────────┐\x1b[0m");
        println!("\x1b[38;2;0;120;255m│                PROMETHEUS OS — AI COMMAND CENTER              │\x1b[0m");
        println!("\x1b[38;2;0;120;255m└─────────────────────────────────────────────────────────────────┘\x1b[0m");

        let cpu = m.cpu();
        let mem = m.memory();
        let gpu = m.gpu();

        println!("\n\x1b[38;2;0;120;255mCPU\x1b[0m  {:.1}%  {:.0}°C  {:.0}MHz  [{:.2} {:.2} {:.2}]",
            cpu.usage_percent, cpu.temperature, cpu.frequency,
            cpu.load[0], cpu.load[1], cpu.load[2]);

        println!("\x1b[38;2;0;120;255mRAM\x1b[0m  {:.1}GB / {:.1}GB ({:.0}%)",
            mem.used_gb, mem.total_gb,
            if mem.total_gb > 0.0 { mem.used_gb / mem.total_gb * 100.0 } else { 0.0 });

        if !gpu.name.is_empty() {
            println!("\x1b[38;2;0;120;255mGPU\x1b[0m  {}  {:.1}%  {:.0}°C  VRAM: {}MB / {}MB",
                gpu.name, gpu.usage_percent, gpu.temperature,
                gpu.vram_used_mb, gpu.vram_total_mb);
        }

        let temps = m.temperatures();
        for t in &temps {
            println!("\x1b[38;2;0;120;255mTEMP\x1b[0m {}  {:.0}°C", t.name, t.temp_c);
        }

        println!("\n\x1b[2mPress 'q' to quit\x1b[0m");
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
}
