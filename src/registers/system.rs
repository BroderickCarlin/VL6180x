//! System Registers (0x010 - 0x017)
//!
//! These registers contain system configuration including GPIO, interrupts,
//! fresh out of reset flag, and history buffer settings.

use core::convert::Infallible;
use regiface::{register, FromByteArray, ReadableRegister, ToByteArray, WritableRegister};

use crate::types::{GpioFunction, GpioPolarity, InterruptMode};

/// GPIO0 Mode Register (0x010)
///
/// Configures the function and polarity of GPIO0 pin.
#[register(0x0010u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ModeGpio0 {
    /// GPIO0 function select
    pub function: GpioFunction,
    /// GPIO0 polarity
    pub polarity: GpioPolarity,
}

impl FromByteArray for ModeGpio0 {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let function = if bytes[0] & 0x10 != 0 {
            GpioFunction::InterruptOutput
        } else {
            GpioFunction::Off
        };

        let polarity = if bytes[0] & 0x01 != 0 {
            GpioPolarity::ActiveHigh
        } else {
            GpioPolarity::ActiveLow
        };

        Ok(Self { function, polarity })
    }
}

impl ToByteArray for ModeGpio0 {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let function_bit = match self.function {
            GpioFunction::Off => 0x00,
            GpioFunction::InterruptOutput => 0x10,
        };

        let polarity_bit = match self.polarity {
            GpioPolarity::ActiveLow => 0x00,
            GpioPolarity::ActiveHigh => 0x01,
        };

        Ok([function_bit | polarity_bit])
    }
}

/// GPIO1 Mode Register (0x011)
///
/// Configures the function and polarity of GPIO1 pin.
#[register(0x0011u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ModeGpio1 {
    /// GPIO1 function select
    pub function: GpioFunction,
    /// GPIO1 polarity
    pub polarity: GpioPolarity,
}

impl FromByteArray for ModeGpio1 {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let function = if bytes[0] & 0x10 != 0 {
            GpioFunction::InterruptOutput
        } else {
            GpioFunction::Off
        };

        let polarity = if bytes[0] & 0x01 != 0 {
            GpioPolarity::ActiveHigh
        } else {
            GpioPolarity::ActiveLow
        };

        Ok(Self { function, polarity })
    }
}

impl ToByteArray for ModeGpio1 {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let function_bit = match self.function {
            GpioFunction::Off => 0x00,
            GpioFunction::InterruptOutput => 0x10,
        };

        let polarity_bit = match self.polarity {
            GpioPolarity::ActiveLow => 0x00,
            GpioPolarity::ActiveHigh => 0x01,
        };

        Ok([function_bit | polarity_bit])
    }
}

/// History Control Register (0x012)
///
/// Controls the history buffer for averaging measurements.
#[register(0x0012u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HistoryCtrl {
    /// Enable history buffer
    pub enable: bool,
    /// Clear history buffer
    pub clear: bool,
}

impl FromByteArray for HistoryCtrl {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            enable: bytes[0] & 0x01 != 0,
            clear: bytes[0] & 0x02 != 0,
        })
    }
}

impl ToByteArray for HistoryCtrl {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let mut value = 0u8;
        if self.enable {
            value |= 0x01;
        }
        if self.clear {
            value |= 0x02;
        }
        Ok([value])
    }
}

/// Interrupt Configuration GPIO Register (0x014)
///
/// Configures interrupt modes for range and ALS measurements.
#[register(0x0014u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InterruptConfigGpio {
    /// Range interrupt mode
    pub range_interrupt: InterruptMode,
    /// ALS interrupt mode
    pub als_interrupt: InterruptMode,
}

impl FromByteArray for InterruptConfigGpio {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        let range_mode = (bytes[0] >> 3) & 0x07;
        let als_mode = bytes[0] & 0x07;

        let range_interrupt =
            InterruptMode::try_from(range_mode).unwrap_or(InterruptMode::Disabled);
        let als_interrupt = InterruptMode::try_from(als_mode).unwrap_or(InterruptMode::Disabled);

        Ok(Self {
            range_interrupt,
            als_interrupt,
        })
    }
}

impl ToByteArray for InterruptConfigGpio {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let range_bits = (self.range_interrupt as u8) << 3;
        let als_bits = self.als_interrupt as u8;
        Ok([range_bits | als_bits])
    }
}

/// Interrupt Clear Register (0x015)
///
/// Writing to this register clears interrupt status flags.
#[register(0x0015u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InterruptClear {
    /// Clear range interrupt
    pub clear_range: bool,
    /// Clear ALS interrupt
    pub clear_als: bool,
    /// Clear error interrupt
    pub clear_error: bool,
}

impl FromByteArray for InterruptClear {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            clear_range: bytes[0] & 0x01 != 0,
            clear_als: bytes[0] & 0x02 != 0,
            clear_error: bytes[0] & 0x04 != 0,
        })
    }
}

impl ToByteArray for InterruptClear {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let mut value = 0u8;
        if self.clear_range {
            value |= 0x01;
        }
        if self.clear_als {
            value |= 0x02;
        }
        if self.clear_error {
            value |= 0x04;
        }
        Ok([value])
    }
}

/// Fresh Out of Reset Register (0x016)
///
/// This register indicates if the device has been reset.
/// Value is 1 after power-on or reset, and should be cleared by software.
#[register(0x0016u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FreshOutOfReset {
    /// Fresh out of reset flag (1 = fresh reset, 0 = cleared)
    pub fresh: bool,
}

impl FromByteArray for FreshOutOfReset {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            fresh: bytes[0] & 0x01 != 0,
        })
    }
}

impl ToByteArray for FreshOutOfReset {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok([if self.fresh { 0x01 } else { 0x00 }])
    }
}

/// Grouped Parameter Hold Register (0x017)
///
/// Controls whether parameter updates are grouped or immediate.
#[register(0x0017u16)]
#[derive(Debug, Clone, Copy, ReadableRegister, WritableRegister)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GroupedParameterHold {
    /// Hold parameter updates (true = hold, false = immediate)
    pub hold: bool,
}

impl FromByteArray for GroupedParameterHold {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            hold: bytes[0] & 0x01 != 0,
        })
    }
}

impl ToByteArray for GroupedParameterHold {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok([if self.hold { 0x01 } else { 0x00 }])
    }
}
