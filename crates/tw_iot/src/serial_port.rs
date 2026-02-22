//! Serial port abstraction for communicating with Raspberry Pi boards.

/// Serial connection state.
#[derive(Debug, Clone)]
pub struct SerialConnection {
    pub port_name:  String,
    pub baud_rate:  u32,
    pub connected:  bool,
    pub rx_buffer:  Vec<String>,
    pub tx_log:     Vec<String>,
}

impl SerialConnection {
    pub fn new() -> Self {
        Self {
            port_name: String::new(),
            baud_rate: 115200,
            connected: false,
            rx_buffer: Vec::new(),
            tx_log:    Vec::new(),
        }
    }

    /// List available serial ports (simulated on non-Pi platforms).
    pub fn available_ports() -> Vec<String> {
        #[cfg(all(feature = "serial", target_os = "linux"))]
        {
            if let Ok(ports) = serialport::available_ports() {
                return ports.iter().map(|p| p.port_name.clone()).collect();
            }
        }
        // Simulated ports for development
        vec![
            "/dev/ttyUSB0 (simulated)".to_string(),
            "/dev/ttyACM0 (simulated)".to_string(),
        ]
    }

    /// Connect to a serial port.
    pub fn connect(&mut self, port: &str, baud: u32) -> Result<(), String> {
        self.port_name = port.to_string();
        self.baud_rate = baud;
        self.connected = true;
        self.rx_buffer.push(format!("Connected to {} at {} baud", port, baud));
        Ok(())
    }

    /// Disconnect.
    pub fn disconnect(&mut self) {
        self.connected = false;
        self.rx_buffer.push("Disconnected".to_string());
    }

    /// Send data.
    pub fn send(&mut self, data: &str) -> Result<(), String> {
        if !self.connected {
            return Err("Not connected".to_string());
        }
        self.tx_log.push(data.to_string());
        Ok(())
    }
}

impl Default for SerialConnection {
    fn default() -> Self { Self::new() }
}
