pub struct SerialManager {
    ports: Vec<SerialPort>,
}

struct SerialPort {
    path: String,
    baud: u32,
    open: bool,
}

impl SerialManager {
    pub fn new() -> Self {
        Self {
            ports: Vec::new(),
        }
    }

    pub fn list_ports(&self) -> Vec<String> {
        vec![]
    }

    pub fn open(&mut self, path: &str, baud: u32) -> anyhow::Result<()> {
        self.ports.push(SerialPort {
            path: path.to_string(),
            baud,
            open: true,
        });
        Ok(())
    }

    pub fn read(&self, _port: &str) -> anyhow::Result<Vec<u8>> {
        Ok(Vec::new())
    }

    pub fn write(&self, _port: &str, data: &[u8]) -> anyhow::Result<()> {
        Ok(())
    }
}
