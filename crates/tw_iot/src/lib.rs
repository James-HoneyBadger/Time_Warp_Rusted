//! IoT / Raspberry Pi GPIO abstraction layer for Time Warp Studio.
//!
//! Provides a cross-platform GPIO interface that:
//! - Uses `rppal` for real GPIO on Raspberry Pi (behind `gpio` feature)
//! - Falls back to a software simulator on non-Pi platforms
//! - Exposes a unified API that language interpreters can call

pub mod gpio;
pub mod board;
pub mod serial_port;

pub use gpio::{GpioManager, PinMode, PinState};
pub use board::Board;
