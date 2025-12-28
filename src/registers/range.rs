//! Range Configuration Registers (0x018 - 0x031)
//!
//! These registers configure the ranging sensor including measurement timing,
//! crosstalk compensation, and convergence settings.

use core::convert::Infallible;
use jiff::Span;
use measurements::Length;
use regiface::{register, FromByteArray, ReadableRegister, ToByteArray, WritableRegister};

/// Range Start Register (0x018)
///
/// Writing to this register starts a range measurement.
#[register(0x0018u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RangeStart {
    /// Single-shot ranging mode (0x01)
    SingleShot,
    /// Continuous ranging mode (0x03)
    Continuous,
}

impl FromByteArray for RangeStart {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(match bytes[0] {
            0x01 => Self::SingleShot,
            0x03 => Self::Continuous,
            _ => Self::SingleShot, // Default to single-shot for unknown values
        })
    }
}

impl ToByteArray for RangeStart {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let value = match self {
            Self::SingleShot => 0x01,
            Self::Continuous => 0x03,
        };
        Ok([value])
    }
}

/// Range Thresholds Register (0x019-0x01C)
///
/// Combined high and low thresholds for range interrupt generation.
/// Reads 4 bytes: threshold_high_hi, threshold_high_lo, threshold_low_hi, threshold_low_lo
#[register(0x0019u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeThresholds {
    /// High threshold
    pub high: Length,
    /// Low threshold
    pub low: Length,
}

impl FromByteArray for RangeThresholds {
    type Error = Infallible;
    type Array = [u8; 4];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let high_mm = u16::from_be_bytes([bytes[0], bytes[1]]);
        let low_mm = u16::from_be_bytes([bytes[2], bytes[3]]);

        Ok(Self {
            high: Length::from_millimeters(high_mm as f64),
            low: Length::from_millimeters(low_mm as f64),
        })
    }
}

impl ToByteArray for RangeThresholds {
    type Error = Infallible;
    type Array = [u8; 4];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let high_mm = self.high.as_millimeters() as u16;
        let low_mm = self.low.as_millimeters() as u16;

        let mut result = [0u8; 4];
        result[0..2].copy_from_slice(&high_mm.to_be_bytes());
        result[2..4].copy_from_slice(&low_mm.to_be_bytes());
        Ok(result)
    }
}

/// Range Intermeasurement Period Register (0x01B)
///
/// Time delay between measurements in continuous mode.
#[register(0x001Bu16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeIntermeasurementPeriod {
    /// Period between measurements
    pub period: Span,
}

impl FromByteArray for RangeIntermeasurementPeriod {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        // Period is in units of 10ms: value 0 = 10ms, value 1 = 20ms, etc.
        let milliseconds = (bytes[0] as i64 + 1) * 10;
        let period = Span::new().milliseconds(milliseconds);
        Ok(Self { period })
    }
}

impl ToByteArray for RangeIntermeasurementPeriod {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        // Convert back to units of 10ms
        let total_ms = self.period.get_milliseconds();
        let value = ((total_ms / 10) - 1).max(0).min(255) as u8;
        Ok([value])
    }
}

/// Range Max Convergence Time Register (0x01C)
///
/// Maximum time to run measurement in ranging modes (up to 63ms).
#[register(0x001Cu16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeMaxConvergenceTime {
    /// Maximum convergence time
    pub time: Span,
}

impl FromByteArray for RangeMaxConvergenceTime {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let time = Span::new().milliseconds(bytes[0] as i64);
        Ok(Self { time })
    }
}

impl ToByteArray for RangeMaxConvergenceTime {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let ms = self.time.get_milliseconds().max(1).min(63) as u8;
        Ok([ms])
    }
}

/// Range Crosstalk Compensation Rate Register (0x01E-0x01F)
///
/// Crosstalk compensation value (9.7 fixed point format).
#[register(0x001Eu16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeCrosstalkCompensationRate {
    /// Crosstalk compensation rate (9.7 fixed point)
    pub rate: u16,
}

impl FromByteArray for RangeCrosstalkCompensationRate {
    type Error = Infallible;
    type Array = [u8; 2];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let rate = u16::from_be_bytes([bytes[0], bytes[1]]);
        Ok(Self { rate })
    }
}

impl ToByteArray for RangeCrosstalkCompensationRate {
    type Error = Infallible;
    type Array = [u8; 2];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok(self.rate.to_be_bytes())
    }
}

/// Range Crosstalk Valid Height Register (0x021)
///
/// Minimum range value to use for crosstalk compensation.
#[register(0x0021u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeCrosstalkValidHeight {
    /// Minimum valid height
    pub height: Length,
}

impl FromByteArray for RangeCrosstalkValidHeight {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            height: Length::from_millimeters(bytes[0] as f64),
        })
    }
}

impl ToByteArray for RangeCrosstalkValidHeight {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let mm = self.height.as_millimeters() as u8;
        Ok([mm])
    }
}

/// Range Early Convergence Estimate Register (0x022-0x023)
///
/// Early convergence estimate threshold (9.7 fixed point format).
#[register(0x0022u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeEarlyConvergenceEstimate {
    /// Early convergence estimate (9.7 fixed point)
    pub estimate: u16,
}

impl FromByteArray for RangeEarlyConvergenceEstimate {
    type Error = Infallible;
    type Array = [u8; 2];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let estimate = u16::from_be_bytes([bytes[0], bytes[1]]);
        Ok(Self { estimate })
    }
}

impl ToByteArray for RangeEarlyConvergenceEstimate {
    type Error = Infallible;
    type Array = [u8; 2];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok(self.estimate.to_be_bytes())
    }
}

/// Range Check Enables Register (0x02D)
///
/// Enable/disable various range check features.
#[register(0x002Du16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeCheckEnables {
    /// Enable range check for signal to noise ratio
    pub enable_snr_check: bool,
    /// Enable range check for range value
    pub enable_range_check: bool,
    /// Enable early convergence estimate check
    pub enable_early_convergence_check: bool,
}

impl FromByteArray for RangeCheckEnables {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            enable_snr_check: bytes[0] & 0x01 != 0,
            enable_range_check: bytes[0] & 0x02 != 0,
            enable_early_convergence_check: bytes[0] & 0x04 != 0,
        })
    }
}

impl ToByteArray for RangeCheckEnables {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let mut value = 0u8;
        if self.enable_snr_check {
            value |= 0x01;
        }
        if self.enable_range_check {
            value |= 0x02;
        }
        if self.enable_early_convergence_check {
            value |= 0x04;
        }
        Ok([value])
    }
}

/// Range VHV Recalibrate Register (0x02E)
///
/// Controls VHV (Vertical Horizontal Vertical) recalibration.
#[register(0x002Eu16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeVhvRecalibrate {
    /// VHV recalibrate value
    pub recalibrate: u8,
}

impl FromByteArray for RangeVhvRecalibrate {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            recalibrate: bytes[0],
        })
    }
}

impl ToByteArray for RangeVhvRecalibrate {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok([self.recalibrate])
    }
}

/// Range VHV Repeat Rate Register (0x031)
///
/// Rate at which VHV recalibration is performed.
#[register(0x0031u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RangeVhvRepeatRate {
    /// VHV repeat rate value
    pub rate: u8,
}

impl FromByteArray for RangeVhvRepeatRate {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self { rate: bytes[0] })
    }
}

impl ToByteArray for RangeVhvRepeatRate {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok([self.rate])
    }
}
