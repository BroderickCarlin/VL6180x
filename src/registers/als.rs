//! ALS (Ambient Light Sensor) Configuration Registers (0x038 - 0x050)
//!
//! These registers configure the ambient light sensor including gain,
//! integration time, and thresholds.

use core::{convert::Infallible, time::Duration};
use regiface::{register, FromByteArray, ReadableRegister, ToByteArray, WritableRegister};

use crate::types::{AlsGain, Luminance, RegisterError};

/// ALS Start Register (0x038)
///
/// Writing to this register starts an ALS measurement.
#[register(0x0038u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlsStart {
    /// Single-shot ALS mode (0x01)
    SingleShot,
    /// Continuous ALS mode (0x03)
    Continuous,
}

impl FromByteArray for AlsStart {
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

impl ToByteArray for AlsStart {
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

/// ALS Thresholds Register (0x03A-0x03D)
///
/// Combined high and low thresholds for ALS interrupt generation.
/// Reads 4 bytes: threshold_high_hi, threshold_high_lo, threshold_low_hi, threshold_low_lo
#[register(0x003Au16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AlsThresholds {
    /// High threshold
    pub high: Luminance,
    /// Low threshold
    pub low: Luminance,
}

impl FromByteArray for AlsThresholds {
    type Error = Infallible;
    type Array = [u8; 4];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let high_raw = u16::from_be_bytes([bytes[0], bytes[1]]);
        let low_raw = u16::from_be_bytes([bytes[2], bytes[3]]);

        Ok(Self {
            high: Luminance {
                lux: high_raw as f32,
            },
            low: Luminance {
                lux: low_raw as f32,
            },
        })
    }
}

impl ToByteArray for AlsThresholds {
    type Error = Infallible;
    type Array = [u8; 4];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let high_raw = self.high.lux as u16;
        let low_raw = self.low.lux as u16;

        let mut result = [0u8; 4];
        result[0..2].copy_from_slice(&high_raw.to_be_bytes());
        result[2..4].copy_from_slice(&low_raw.to_be_bytes());
        Ok(result)
    }
}

/// ALS Intermeasurement Period Register (0x03E)
///
/// Time delay between measurements in continuous mode.
#[register(0x003Eu16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AlsIntermeasurementPeriod {
    /// Period between measurements
    pub period: Duration,
}

impl FromByteArray for AlsIntermeasurementPeriod {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        // Period is in units of 10ms: value 0 = 10ms, value 1 = 20ms, etc.
        let milliseconds = (bytes[0] as u64 + 1) * 10;
        let period = Duration::from_millis(milliseconds);
        Ok(Self { period })
    }
}

impl ToByteArray for AlsIntermeasurementPeriod {
    type Error = RegisterError;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        // Convert to units of 10ms
        // Valid range: 10ms to 2560ms (value 0-255 in register)
        let total_ms = self.period.as_millis() as u64;

        if total_ms < 10 {
            return Err(RegisterError::DurationTooShort);
        }
        if total_ms > 2560 {
            return Err(RegisterError::DurationTooLong);
        }

        let value = (((total_ms + 5) / 10) - 1) as u8;
        Ok([value])
    }
}

/// ALS Analogue Gain Register (0x03F)
///
/// Configures the ALS analog gain setting.
#[register(0x003Fu16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AlsAnalogueGain {
    /// Analog gain setting
    pub gain: AlsGain,
}

impl FromByteArray for AlsAnalogueGain {
    type Error = RegisterError;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let gain = AlsGain::try_from(bytes[0])?;
        Ok(Self { gain })
    }
}

impl ToByteArray for AlsAnalogueGain {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok([self.gain as u8])
    }
}

/// ALS Integration Period Register (0x040)
///
/// Integration time for the ALS measurement (in ms, 1-100ms typical range).
#[register(0x0040u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AlsIntegrationPeriod {
    /// Integration period
    pub period: Duration,
}

impl FromByteArray for AlsIntegrationPeriod {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        // Integration period is in milliseconds
        let period = Duration::from_millis(bytes[0] as u64);
        Ok(Self { period })
    }
}

impl ToByteArray for AlsIntegrationPeriod {
    type Error = RegisterError;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        // Valid range: 1ms to 255ms
        let ms = self.period.as_millis() as u64;

        if ms < 1 {
            return Err(RegisterError::DurationTooShort);
        }
        if ms > 255 {
            return Err(RegisterError::DurationTooLong);
        }

        Ok([ms as u8])
    }
}
