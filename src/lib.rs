#![no_std]
//! VL6180X Proximity and Ambient Light Sensor Driver
//!
//! This crate provides a type-safe interface for the ST VL6180X proximity and ambient light sensor.
//! The VL6180X is a Time-of-Flight (ToF) ranging sensor with integrated ambient light sensor (ALS)
//! that can measure absolute distances up to 100mm independent of target reflectance.
//!
//! Basic usage follows this pattern:
//!
//! 1. Create a new [`Device`] instance with your I2C interface
//! 2. Verify device identification
//! 3. Configure sensor settings (range, ALS, interrupts)
//! 4. Perform measurements (single-shot or continuous)
//! 5. Read measurement results
//!
//! # Example
//! ```no_run
//! use embedded_hal::i2c::I2c;
//! use vl6180x::{Device, registers::ModelId};
//!
//! fn configure_sensor<I2C: I2c>(i2c: I2C) -> Result<Device<I2C>, vl6180x::Error> {
//!     let mut device = Device::new(i2c);
//!     
//!     // Read device model ID (should be 0xB4)
//!     let model_id: ModelId = device.read_register()?;
//!     
//!     Ok(device)
//! }
//! ```

pub use regiface::errors::Error;

pub mod device;
pub mod registers;
pub mod types;

pub use device::Device;
pub use types::*;
