//! VL6180X Device Interface
//!
//! This module provides the main interface for interacting with VL6180X devices
//! through I2C communication. It supports both blocking and asynchronous operations.

use regiface::{errors::Error as RegifaceError, ByteArray, ReadableRegister, WritableRegister};

/// Default I2C address for the VL6180X (7-bit)
pub const DEFAULT_ADDRESS: u8 = 0x29;

/// Main device interface for the VL6180X sensor.
///
/// This struct wraps an I2C interface and provides methods to interact with the sensor.
/// It supports both blocking operations through the embedded-hal traits and
/// asynchronous operations through embedded-hal-async.
///
/// The VL6180X uses 16-bit register addresses.
pub struct Device<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> Device<I2C> {
    /// Creates a new Device instance with the default I2C address (0x29).
    ///
    /// # Arguments
    /// * `i2c` - An I2C interface implementing the required embedded-hal traits
    pub fn new(i2c: I2C) -> Self {
        Self::new_with_address(i2c, DEFAULT_ADDRESS)
    }

    /// Creates a new Device instance with a custom I2C address.
    ///
    /// # Arguments
    /// * `i2c` - An I2C interface implementing the required embedded-hal traits
    /// * `address` - Custom 7-bit I2C address
    pub fn new_with_address(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    /// Releases the underlying I2C device.
    ///
    /// This method consumes the Device instance and returns the wrapped I2C interface.
    pub fn release(self) -> I2C {
        self.i2c
    }
}

impl<I2C> Device<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    /// Reads a register value from the device.
    ///
    /// # Type Parameters
    /// * `R` - Register type implementing ReadableRegister with u16 ID
    ///
    /// # Errors
    /// * `RegifaceError::BusError` - I2C communication failed
    /// * `RegifaceError::DeserializationError` - Failed to parse register value
    pub fn read_register<R>(&mut self) -> Result<R, RegifaceError>
    where
        R: ReadableRegister<IdType = u16>,
    {
        let reg_addr = R::id().to_be_bytes();
        let mut buf = R::Array::new();

        self.i2c
            .write_read(self.address, &reg_addr, buf.as_mut())
            .map_err(|_| RegifaceError::BusError)?;

        R::from_bytes(buf).map_err(|_| RegifaceError::DeserializationError)
    }

    /// Writes a value to a device register.
    ///
    /// # Type Parameters
    /// * `R` - Register type implementing WritableRegister with u16 ID
    ///
    /// # Arguments
    /// * `register` - The register value to write
    ///
    /// # Errors
    /// * `RegifaceError::BusError` - I2C communication failed
    /// * `RegifaceError::SerializationError` - Failed to serialize register value
    pub fn write_register<R>(&mut self, register: R) -> Result<(), RegifaceError>
    where
        R: WritableRegister<IdType = u16>,
    {
        let reg_addr = R::id().to_be_bytes();
        let value = register
            .to_bytes()
            .map_err(|_| RegifaceError::SerializationError)?;

        self.i2c
            .transaction(
                self.address,
                &mut [
                    embedded_hal::i2c::Operation::Write(&reg_addr),
                    embedded_hal::i2c::Operation::Write(value.as_ref()),
                ],
            )
            .map_err(|_| RegifaceError::BusError)
    }
}

impl<I2C> Device<I2C>
where
    I2C: embedded_hal_async::i2c::I2c,
{
    /// Asynchronously reads a register value from the device.
    ///
    /// This is the async version of [`read_register`](Device::read_register).
    pub async fn read_register_async<R>(&mut self) -> Result<R, RegifaceError>
    where
        R: ReadableRegister<IdType = u16>,
    {
        let reg_addr = R::id().to_be_bytes();
        let mut buf = R::Array::new();

        self.i2c
            .write_read(self.address, &reg_addr, buf.as_mut())
            .await
            .map_err(|_| RegifaceError::BusError)?;

        R::from_bytes(buf).map_err(|_| RegifaceError::DeserializationError)
    }

    /// Asynchronously writes a value to a device register.
    ///
    /// This is the async version of [`write_register`](Device::write_register).
    pub async fn write_register_async<R>(&mut self, register: R) -> Result<(), RegifaceError>
    where
        R: WritableRegister<IdType = u16>,
    {
        let reg_addr = R::id().to_be_bytes();
        let value = register
            .to_bytes()
            .map_err(|_| RegifaceError::SerializationError)?;

        self.i2c
            .transaction(
                self.address,
                &mut [
                    embedded_hal_async::i2c::Operation::Write(&reg_addr),
                    embedded_hal_async::i2c::Operation::Write(value.as_ref()),
                ],
            )
            .await
            .map_err(|_| RegifaceError::BusError)
    }
}
