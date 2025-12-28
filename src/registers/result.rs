//! Result Registers (0x04D - 0x066)
//!
//! These registers contain measurement results from both the ranging
//! and ambient light sensors.

use jiff::Span;
use measurements::Length;
use regiface::{register, FromByteArray, ReadableRegister};

use crate::types::{AlsErrorCode, RangeErrorCode};

/// Range Result Value Register (0x062)
///
/// Range measurement result.
#[register(0x0062u16)]
#[derive(Debug, Clone, Copy, ReadableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeResultValue {
    /// Measured distance
    pub distance: Length,
}

impl FromByteArray for RangeResultValue {
    type Error = core::convert::Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            distance: Length::from_millimeters(bytes[0] as f64),
        })
    }
}

/// Range Result Status Register (0x04D)
///
/// Contains range error code and device ready status.
#[register(0x004Du16)]
#[derive(Debug, Clone, Copy, ReadableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeResultStatus {
    /// Range error code
    pub error_code: RangeErrorCode,
    /// Device ready for new command
    pub device_ready: bool,
}

impl FromByteArray for RangeResultStatus {
    type Error = ();
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let error_code = RangeErrorCode::try_from((bytes[0] >> 4) & 0x0F)?;
        let device_ready = bytes[0] & 0x01 != 0;

        Ok(Self {
            error_code,
            device_ready,
        })
    }
}

/// Result Interrupt Status GPIO Register (0x04F)
///
/// Interrupt status bits for range, ALS, and error interrupts.
#[register(0x004Fu16)]
#[derive(Debug, Clone, Copy, ReadableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResultInterruptStatusGpio {
    /// Range interrupt status
    pub range_interrupt: bool,
    /// ALS interrupt status
    pub als_interrupt: bool,
    /// Error interrupt status
    pub error_interrupt: bool,
}

impl FromByteArray for ResultInterruptStatusGpio {
    type Error = core::convert::Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            range_interrupt: bytes[0] & 0x04 != 0,
            als_interrupt: bytes[0] & 0x08 != 0,
            error_interrupt: bytes[0] & 0x10 != 0,
        })
    }
}

/// ALS Result Value Register (0x050-0x051)
///
/// ALS measurement result (16-bit raw count value).
#[register(0x0050u16)]
#[derive(Debug, Clone, Copy, ReadableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AlsResultValue {
    /// Measured ambient light level (raw counts)
    pub raw_count: u16,
}

impl FromByteArray for AlsResultValue {
    type Error = core::convert::Infallible;
    type Array = [u8; 2];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let raw_count = u16::from_be_bytes([bytes[0], bytes[1]]);
        Ok(Self { raw_count })
    }
}

/// Result ALS Status Register (0x04E)
///
/// ALS status and error information.
#[register(0x004Eu16)]
#[derive(Debug, Clone, Copy, ReadableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResultAlsStatus {
    /// ALS error code
    pub error_code: AlsErrorCode,
    /// Device ready for new command
    pub device_ready: bool,
}

impl FromByteArray for ResultAlsStatus {
    type Error = ();
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let error_code = AlsErrorCode::try_from((bytes[0] >> 4) & 0x0F)?;
        let device_ready = bytes[0] & 0x01 != 0;

        Ok(Self {
            error_code,
            device_ready,
        })
    }
}

/// Range Result Convergence Time Register (0x063-0x066)
///
/// Convergence time for the range measurement
#[register(0x0063u16)]
#[derive(Debug, Clone, Copy, ReadableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeResultConvergenceTime {
    /// Convergence time
    pub time: Span,
}

impl FromByteArray for RangeResultConvergenceTime {
    type Error = core::convert::Infallible;
    type Array = [u8; 4];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let time_ms = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let time = Span::new().milliseconds(time_ms as i64);
        Ok(Self { time })
    }
}
