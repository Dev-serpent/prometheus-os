pub struct DesktopAPI;

impl DesktopAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn open_file(&self, _path: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn open_url(&self, _url: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn send_notification(&self, title: &str, body: &str) -> anyhow::Result<()> {
        println!("[Notification] {}: {}", title, body);
        Ok(())
    }

    pub fn set_clipboard(&self, _text: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn get_clipboard(&self) -> anyhow::Result<String> {
        Ok(String::new())
    }
}
