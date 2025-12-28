//! Register definitions for the VL6180X
//!
//! This module contains all register definitions organized by function:
//! - Identification: Device ID and version information
//! - System: GPIO, interrupts, and control registers
//! - Range: Ranging sensor configuration
//! - ALS: Ambient light sensor configuration
//! - Result: Measurement results

mod als;
mod identification;
mod range;
mod result;
mod system;

pub use als::*;
pub use identification::*;
pub use range::*;
pub use result::*;
pub use system::*;
