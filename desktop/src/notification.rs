pub struct NotificationCenter;

impl NotificationCenter {
    pub fn new() -> Self { Self }

    pub fn send(&self, title: &str, body: &str) {
        println!("[Notification] {}: {}", title, body);
    }

    pub fn run(&self) -> anyhow::Result<()> { Ok(()) }
}
