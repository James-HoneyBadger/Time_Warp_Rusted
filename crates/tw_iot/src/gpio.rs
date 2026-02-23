//! GPIO abstraction — simulated on non-Pi platforms, real on Raspberry Pi.

use std::collections::HashMap;

use crate::board::Board;

/// Pin direction / mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinMode {
    Input,
    Output,
    Pwm,
    I2c,
    Spi,
    Unset,
}

impl std::fmt::Display for PinMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PinMode::Input  => write!(f, "IN"),
            PinMode::Output => write!(f, "OUT"),
            PinMode::Pwm    => write!(f, "PWM"),
            PinMode::I2c    => write!(f, "I2C"),
            PinMode::Spi    => write!(f, "SPI"),
            PinMode::Unset  => write!(f, "—"),
        }
    }
}

/// Digital pin state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinState {
    High,
    Low,
    Unknown,
}

impl std::fmt::Display for PinState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PinState::High    => write!(f, "HIGH"),
            PinState::Low     => write!(f, "LOW"),
            PinState::Unknown => write!(f, "—"),
        }
    }
}

/// Per-pin information.
#[derive(Debug, Clone)]
pub struct PinInfo {
    pub number: u8,
    pub mode:   PinMode,
    pub state:  PinState,
    pub value:  f64,       // analog value (0.0–1.0 for ADC, duty for PWM)
    pub label:  String,    // user-assigned label
}

impl PinInfo {
    pub fn new(number: u8) -> Self {
        Self {
            number,
            mode:  PinMode::Unset,
            state: PinState::Unknown,
            value: 0.0,
            label: String::new(),
        }
    }
}

/// Cross-platform GPIO manager.
///
/// On Raspberry Pi with the `gpio` feature, this wraps `rppal::gpio::Gpio`.
/// Otherwise it provides a software simulator suitable for learning and testing.
pub struct GpioManager {
    pub board:    Board,
    pub pins:     HashMap<u8, PinInfo>,
    pub connected: bool,
    pub log:      Vec<String>,
}

impl GpioManager {
    /// Create a new GPIO manager for the given board.
    pub fn new(board: Board) -> Self {
        let mut pins = HashMap::new();
        for i in 0..board.gpio_count() {
            pins.insert(i, PinInfo::new(i));
        }
        Self {
            board,
            pins,
            connected: false,
            log: Vec::new(),
        }
    }

    /// Attempt to connect to real hardware (only works on Pi with feature).
    pub fn connect(&mut self) -> Result<(), String> {
        #[cfg(all(
            feature = "gpio",
            target_os = "linux",
            any(target_arch = "arm", target_arch = "aarch64")
        ))]
        {
            // Real GPIO initialization would happen here
            self.connected = true;
            self.log.push(format!("Connected to {}", self.board));
            return Ok(());
        }

        // Simulator mode
        #[allow(unreachable_code)]
        {
            self.connected = true;
            self.log.push(format!("Simulator mode: {}", self.board));
            Ok(())
        }
    }

    /// Set a pin's mode.
    pub fn pin_mode(&mut self, pin: u8, mode: PinMode) -> Result<(), String> {
        let info = self.pins.get_mut(&pin)
            .ok_or_else(|| format!("Invalid pin: GP{pin}"))?;
        info.mode = mode;
        info.state = match mode {
            PinMode::Output => PinState::Low,
            PinMode::Input  => PinState::Unknown,
            _               => PinState::Unknown,
        };
        self.log.push(format!("GP{pin} set to {mode}"));
        Ok(())
    }

    /// Write a digital value to an output pin.
    pub fn digital_write(&mut self, pin: u8, high: bool) -> Result<(), String> {
        let info = self.pins.get_mut(&pin)
            .ok_or_else(|| format!("Invalid pin: GP{pin}"))?;
        if info.mode != PinMode::Output {
            return Err(format!("GP{pin} is not set to OUTPUT mode"));
        }
        info.state = if high { PinState::High } else { PinState::Low };
        info.value = if high { 1.0 } else { 0.0 };
        self.log.push(format!("GP{pin} = {}", info.state));
        Ok(())
    }

    /// Read a digital value from an input pin.
    pub fn digital_read(&mut self, pin: u8) -> Result<bool, String> {
        let info = self.pins.get(&pin)
            .ok_or_else(|| format!("Invalid pin: GP{pin}"))?;
        if info.mode != PinMode::Input {
            return Err(format!("GP{pin} is not set to INPUT mode"));
        }
        // In simulator, pins start as LOW unless manually toggled
        Ok(info.state == PinState::High)
    }

    /// Set PWM duty cycle (0.0–1.0) on a pin.
    pub fn pwm_write(&mut self, pin: u8, duty: f64) -> Result<(), String> {
        let info = self.pins.get_mut(&pin)
            .ok_or_else(|| format!("Invalid pin: GP{pin}"))?;
        if info.mode != PinMode::Pwm {
            return Err(format!("GP{pin} is not set to PWM mode"));
        }
        let duty = duty.clamp(0.0, 1.0);
        info.value = duty;
        info.state = if duty > 0.0 { PinState::High } else { PinState::Low };
        self.log.push(format!("GP{pin} PWM duty = {:.1}%", duty * 100.0));
        Ok(())
    }

    /// Read analog value (0.0–1.0) from an ADC-capable pin (Pico only).
    pub fn analog_read(&self, pin: u8) -> Result<f64, String> {
        let info = self.pins.get(&pin)
            .ok_or_else(|| format!("Invalid pin: GP{pin}"))?;
        if !self.board.has_adc() {
            return Err(format!("{} does not have ADC", self.board));
        }
        Ok(info.value)
    }

    /// Simulate toggling an input pin (for the GUI simulator).
    pub fn sim_toggle(&mut self, pin: u8) {
        if let Some(info) = self.pins.get_mut(&pin) {
            if info.mode == PinMode::Input {
                info.state = match info.state {
                    PinState::High => PinState::Low,
                    _ => PinState::High,
                };
                info.value = if info.state == PinState::High { 1.0 } else { 0.0 };
                self.log.push(format!("GP{pin} toggled → {}", info.state));
            }
        }
    }

    /// Reset all pins to default state.
    pub fn reset(&mut self) {
        for info in self.pins.values_mut() {
            info.mode = PinMode::Unset;
            info.state = PinState::Unknown;
            info.value = 0.0;
        }
        self.log.push("GPIO reset".to_string());
    }

    /// Get sorted list of pin numbers.
    pub fn sorted_pins(&self) -> Vec<u8> {
        let mut pins: Vec<u8> = self.pins.keys().copied().collect();
        pins.sort();
        pins
    }
}
