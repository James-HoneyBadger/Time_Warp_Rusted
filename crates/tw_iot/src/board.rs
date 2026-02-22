//! Board definitions for supported Raspberry Pi variants.

use std::fmt;

/// Supported Raspberry Pi board types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Board {
    Pico,
    PicoW,
    PiZero,
    PiZero2W,
    Pi4,
    Pi5,
    Simulator,
}

impl Board {
    /// Number of user GPIO pins available.
    pub fn gpio_count(&self) -> u8 {
        match self {
            Board::Pico | Board::PicoW => 26,   // GP0-GP25
            Board::PiZero | Board::PiZero2W => 28,
            Board::Pi4 | Board::Pi5 => 28,
            Board::Simulator => 28,
        }
    }

    /// Human-readable board name.
    pub fn name(&self) -> &'static str {
        match self {
            Board::Pico      => "Raspberry Pi Pico",
            Board::PicoW     => "Raspberry Pi Pico W",
            Board::PiZero    => "Raspberry Pi Zero",
            Board::PiZero2W  => "Raspberry Pi Zero 2 W",
            Board::Pi4       => "Raspberry Pi 4",
            Board::Pi5       => "Raspberry Pi 5",
            Board::Simulator => "GPIO Simulator",
        }
    }

    /// Whether this board has Wi-Fi capability.
    pub fn has_wifi(&self) -> bool {
        matches!(self, Board::PicoW | Board::PiZero2W | Board::Pi4 | Board::Pi5)
    }

    /// Whether this board supports I2C.
    pub fn has_i2c(&self) -> bool { true }

    /// Whether this board supports SPI.
    pub fn has_spi(&self) -> bool { true }

    /// Whether this board supports PWM.
    pub fn has_pwm(&self) -> bool { true }

    /// Whether this board supports ADC (analog-to-digital converter).
    pub fn has_adc(&self) -> bool {
        matches!(self, Board::Pico | Board::PicoW)
    }

    /// All supported boards.
    pub fn all() -> &'static [Board] {
        &[
            Board::Simulator,
            Board::Pico,
            Board::PicoW,
            Board::PiZero,
            Board::PiZero2W,
            Board::Pi4,
            Board::Pi5,
        ]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
