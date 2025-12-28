//! Common types and enumerations for the VL6180X driver

use core::fmt;

/// Unified error type for register operations
///
/// This error type covers all failure modes that can occur during
/// register serialization and deserialization operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterError {
    /// Invalid enum value encountered during deserialization
    /// Contains the raw byte value that was invalid
    InvalidEnumValue(u8),
    /// Duration value is too short for register requirements
    DurationTooShort,
    /// Duration value is too long for register requirements
    DurationTooLong,
    /// Invalid date/time value in timestamp
    InvalidTimestamp,
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEnumValue(value) => write!(f, "Invalid enum value: 0x{:02X}", value),
            Self::DurationTooShort => write!(f, "Duration is too short"),
            Self::DurationTooLong => write!(f, "Duration is too long"),
            Self::InvalidTimestamp => write!(f, "Invalid timestamp"),
        }
    }
}

impl From<jiff::Error> for RegisterError {
    fn from(_: jiff::Error) -> Self {
        Self::InvalidTimestamp
    }
}

/// Luminance measurement in lux
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Luminance {
    pub lux: f32,
}

impl fmt::Display for Luminance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} lux", self.lux)
    }
}

/// ALS error codes
///
/// These error codes are returned in the RESULT__ALS_STATUS register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AlsErrorCode {
    /// No error - valid measurement
    NoError = 0,
    /// Overflow error
    Overflow = 1,
    /// Underflow error
    Underflow = 2,
}

impl TryFrom<u8> for AlsErrorCode {
    type Error = RegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoError),
            1 => Ok(Self::Overflow),
            2 => Ok(Self::Underflow),
            _ => Err(RegisterError::InvalidEnumValue(value)),
        }
    }
}

/// Range error codes from Table 12 of the datasheet
///
/// These error codes are returned in the RESULT__RANGE_STATUS register
/// to indicate various error conditions during range measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum RangeErrorCode {
    /// No error - valid measurement
    NoError = 0,
    /// VCSEL continuity test failure
    VcselContinuityTest = 1,
    /// VCSEL watchdog test failure
    VcselWatchdogTest = 2,
    /// VCSEL watchdog triggered
    VcselWatchdog = 3,
    /// PLL1 lock failure
    Pll1Lock = 4,
    /// PLL2 lock failure
    Pll2Lock = 5,
    /// Early convergence estimate - signal too weak
    EarlyConvergenceEstimate = 6,
    /// Maximum convergence time reached
    MaxConvergence = 7,
    /// Range ignored due to low signal
    NoTargetIgnore = 8,
    /// Signal to noise ratio too low
    SignalToNoiseRatio = 11,
    /// Raw ranging algorithm underflow
    RawRangingUnderflow = 12,
    /// Raw ranging algorithm overflow
    RawRangingOverflow = 13,
    /// Ranging algorithm underflow
    RangingUnderflow = 14,
    /// Ranging algorithm overflow
    RangingOverflow = 15,
}

impl TryFrom<u8> for RangeErrorCode {
    type Error = RegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoError),
            1 => Ok(Self::VcselContinuityTest),
            2 => Ok(Self::VcselWatchdogTest),
            3 => Ok(Self::VcselWatchdog),
            4 => Ok(Self::Pll1Lock),
            5 => Ok(Self::Pll2Lock),
            6 => Ok(Self::EarlyConvergenceEstimate),
            7 => Ok(Self::MaxConvergence),
            8 => Ok(Self::NoTargetIgnore),
            11 => Ok(Self::SignalToNoiseRatio),
            12 => Ok(Self::RawRangingUnderflow),
            13 => Ok(Self::RawRangingOverflow),
            14 => Ok(Self::RangingUnderflow),
            15 => Ok(Self::RangingOverflow),
            _ => Err(RegisterError::InvalidEnumValue(value)),
        }
    }
}

impl RangeErrorCode {
    /// Check if this represents a valid (no error) measurement
    pub const fn is_valid(&self) -> bool {
        matches!(self, Self::NoError)
    }
}

/// ALS (Ambient Light Sensor) analog gain settings
///
/// The VL6180X supports 8 different analog gain settings for the ALS.
/// Higher gain settings provide better sensitivity in low-light conditions
/// but have a reduced maximum measurable light level.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AlsGain {
    /// Gain = 20 (highest gain, lowest max lux)
    Gain20 = 0,
    /// Gain = 10
    Gain10 = 1,
    /// Gain = 5.0
    Gain5 = 2,
    /// Gain = 2.5
    Gain2_5 = 3,
    /// Gain = 1.67
    Gain1_67 = 4,
    /// Gain = 1.25
    Gain1_25 = 5,
    /// Gain = 1.0 (default)
    #[default]
    Gain1 = 6,
    /// Gain = 40 (lowest gain, highest max lux)
    Gain40 = 7,
}

impl TryFrom<u8> for AlsGain {
    type Error = RegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0b111 {
            0 => Ok(Self::Gain20),
            1 => Ok(Self::Gain10),
            2 => Ok(Self::Gain5),
            3 => Ok(Self::Gain2_5),
            4 => Ok(Self::Gain1_67),
            5 => Ok(Self::Gain1_25),
            6 => Ok(Self::Gain1),
            7 => Ok(Self::Gain40),
            _ => Err(RegisterError::InvalidEnumValue(value)),
        }
    }
}

impl AlsGain {
    /// Get the numeric gain value
    pub const fn gain(&self) -> f32 {
        match self {
            Self::Gain20 => 20.0,
            Self::Gain10 => 10.0,
            Self::Gain5 => 5.0,
            Self::Gain2_5 => 2.5,
            Self::Gain1_67 => 1.67,
            Self::Gain1_25 => 1.25,
            Self::Gain1 => 1.0,
            Self::Gain40 => 40.0,
        }
    }
}

/// GPIO polarity configuration
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioPolarity {
    /// Active low (default)
    #[default]
    ActiveLow = 0,
    /// Active high
    ActiveHigh = 1,
}

impl TryFrom<u8> for GpioPolarity {
    type Error = RegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0b1 {
            0 => Ok(Self::ActiveLow),
            1 => Ok(Self::ActiveHigh),
            _ => Err(RegisterError::InvalidEnumValue(value)),
        }
    }
}

/// GPIO function selection
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioFunction {
    /// GPIO is in high-impedance off state (default)
    #[default]
    Off = 0,
    /// GPIO is configured as interrupt output
    InterruptOutput = 1,
}

impl TryFrom<u8> for GpioFunction {
    type Error = RegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0b1 {
            0 => Ok(Self::Off),
            1 => Ok(Self::InterruptOutput),
            _ => Err(RegisterError::InvalidEnumValue(value)),
        }
    }
}

/// Interrupt mode configuration for both ranging and ALS
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum InterruptMode {
    /// Interrupts disabled (default)
    #[default]
    Disabled = 0,
    /// Interrupt when value is below low threshold
    LevelLow = 1,
    /// Interrupt when value is above high threshold
    LevelHigh = 2,
    /// Interrupt when value is outside window (below low OR above high)
    OutOfWindow = 3,
    /// Interrupt on every new sample ready
    NewSampleReady = 4,
}

impl TryFrom<u8> for InterruptMode {
    type Error = RegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0b111 {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::LevelLow),
            2 => Ok(Self::LevelHigh),
            3 => Ok(Self::OutOfWindow),
            4 => Ok(Self::NewSampleReady),
            _ => Err(RegisterError::InvalidEnumValue(value)),
        }
    }
}
