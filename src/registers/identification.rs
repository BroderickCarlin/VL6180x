//! Identification Registers (0x000 - 0x009)
//!
//! These registers contain device identification information including
//! model ID, revision numbers, and manufacturing date/time.

use core::convert::Infallible;
use jiff::civil::DateTime;
use regiface::{register, FromByteArray, ReadableRegister};

use crate::types::RegisterError;

/// Model ID Register (0x000)
///
/// Expected value: VL6180X (0xB4)
/// This register contains the device model identification.
#[register(0x0000u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ReadableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModelId {
    /// VL6180X device (0xB4)
    VL6180X,
    /// Unknown model ID
    Unknown(u8),
}

impl FromByteArray for ModelId {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(match bytes[0] {
            0xB4 => Self::VL6180X,
            value => Self::Unknown(value),
        })
    }
}

/// Model Revision Register (0x001-0x002)
///
/// Combined major and minor model revision numbers.
#[register(0x0001u16)]
#[derive(Debug, Clone, Copy, ReadableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ModelRevision {
    /// Model major revision number
    pub major: u8,
    /// Model minor revision number
    pub minor: u8,
}

impl FromByteArray for ModelRevision {
    type Error = Infallible;
    type Array = [u8; 2];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            major: bytes[0],
            minor: bytes[1],
        })
    }
}

/// Module Revision Register (0x003-0x004)
///
/// Combined major and minor module revision numbers.
#[register(0x0003u16)]
#[derive(Debug, Clone, Copy, ReadableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ModuleRevision {
    /// Module major revision number
    pub major: u8,
    /// Module minor revision number
    pub minor: u8,
}

impl FromByteArray for ModuleRevision {
    type Error = Infallible;
    type Array = [u8; 2];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            major: bytes[0],
            minor: bytes[1],
        })
    }
}

/// Module Manufacturing Timestamp Register (0x006-0x009)
///
/// Combined date and time of manufacture as a DateTime.
/// Reads 4 bytes: date_hi, date_lo, time_hi, time_lo
///
/// Date format:
/// - date_hi: bits [7:4] = year offset from 2010, bits [3:0] = month (1-12)
/// - date_lo: bits [7:3] = day of month (1-31), bits [2:0] = reserved
///
/// Time format:
/// - time_hi:time_lo forms a 16-bit value
/// - This value * 2 = seconds since midnight
#[register(0x0006u16)]
#[derive(Debug, Clone, Copy, ReadableRegister)]
pub struct ModuleTimestamp {
    /// Manufacturing date and time
    pub timestamp: DateTime,
}

impl FromByteArray for ModuleTimestamp {
    type Error = RegisterError;
    type Array = [u8; 4];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        // Parse date bytes
        let date_hi = bytes[0];
        let date_lo = bytes[1];

        // Parse time bytes as 16-bit value
        let time_hi = bytes[2];
        let time_lo = bytes[3];
        let time_value = ((time_hi as u16) << 8) | (time_lo as u16);

        // Extract date components
        let year = 2010 + ((date_hi >> 4) as i16);
        let month = (date_hi & 0x0F) as i8;
        let day = (date_lo >> 3) as i8; // Drop 3 LSB and shift

        // Calculate time components from seconds since midnight
        let seconds_since_midnight = time_value * 2;
        let hour = (seconds_since_midnight / 3600) as i8;
        let minute = ((seconds_since_midnight % 3600) / 60) as i8;
        let second = (seconds_since_midnight % 60) as i8;

        // Create DateTime - will return error if values are invalid
        let timestamp = DateTime::new(year, month, day, hour, minute, second, 0)?;

        Ok(Self { timestamp })
    }
}
