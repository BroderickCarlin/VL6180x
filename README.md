# VL6180X Driver

A no_std, embedded-hal driver for the VL6180X proximity and ambient light sensor.

This driver provides a comprehensive, type-safe interface to the ST VL6180X Time-of-Flight ranging and ambient light sensor. It uses the `regiface` crate for register abstractions and provides both blocking and async I2C communication.

[![Crates.io](https://img.shields.io/crates/v/vl6180x.svg)](https://crates.io/crates/vl6180x)
[![Documentation](https://docs.rs/vl6180x/badge.svg)](https://docs.rs/vl6180x)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
vl6180x = "X"
```

## Examples

### Single-Shot Range Measurement

```rust
use vl6180x::{
    Device,
    registers::{
        FreshOutOfReset, ModelId, 
        RangeStart, RangeResultStatus, RangeResultValue
    },
};

// Create device on the given I2C bus
let mut sensor = Device::new(i2c);

// Check if device is fresh out of reset
let fresh: FreshOutOfReset = sensor.read_register()?;
if fresh.fresh {
    // Clear the fresh out of reset flag
    sensor.write_register(FreshOutOfReset { fresh: false })?;
    
    // Perform any initialization here (configure thresholds, gains, etc.)
}

// Optionally, verify device model ID
let model_id: ModelId = sensor.read_register()?;
match model_id {
    ModelId::VL6180X => {
        // Correct device detected
    }
    ModelId::Unknown(id) => {
        // Handle unexpected device
    }
}

// Start a single-shot range measurement
sensor.write_register(RangeStart::SingleShot)?;

// Poll until measurement is ready and check status
let status = loop {
    let status: RangeResultStatus = sensor.read_register()?;
    if status.device_ready {
        break status;
    }
    delay.delay_ms(10);
};

// Only read result if measurement is valid
if status.error_code.is_valid() {
    let result: RangeResultValue = sensor.read_register()?;
    let distance_mm = result.distance.as_millimeters();
    println!("Distance: {} mm", distance_mm);
} else {
    println!("Error: {:?}", status.error_code);
}
```

### Continuous Range Measurements

```rust
use core::time::Duration;
use vl6180x::{
    Device,
    registers::{
        RangeStart, RangeIntermeasurementPeriod,
        RangeResultStatus, RangeResultValue,
        InterruptClear,
    },
};

let mut sensor = Device::new(i2c);

// Configure intermeasurement period (100ms)
sensor.write_register(RangeIntermeasurementPeriod {
    period: Duration::from_millis(100),
})?;

// Start continuous measurements
sensor.write_register(RangeStart::Continuous)?;

loop {
    // Check status first
    let status: RangeResultStatus = sensor.read_register()?;
    
    if status.device_ready {
        if status.error_code.is_valid() {
            // Only read result if valid
            let result: RangeResultValue = sensor.read_register()?;
            let distance_mm = result.distance.as_millimeters();
            println!("Distance: {} mm", distance_mm);
        } else {
            // Handle error
            println!("Range error: {:?}", status.error_code);
        }
    }
    
    delay.delay_ms(10);
}
```

### Ambient Light Sensing

```rust
use core::time::Duration;
use vl6180x::{
    Device,
    registers::{
        AlsStart, AlsAnalogueGain, AlsIntegrationPeriod,
        AlsResultValue, ResultAlsStatus,
    },
    types::AlsGain,
};

let mut sensor = Device::new(i2c);

// Configure ALS gain
sensor.write_register(AlsAnalogueGain {
    gain: AlsGain::Gain1,
})?;

// Configure integration period (100ms)
sensor.write_register(AlsIntegrationPeriod {
    period: Duration::from_millis(100),
})?;

// Start single-shot ALS measurement
sensor.write_register(AlsStart::SingleShot)?;

// Poll until measurement is ready and check status
let status = loop {
    let status: ResultAlsStatus = sensor.read_register()?;
    if status.device_ready {
        break status;
    }
    delay.delay_ms(10);
};

// Only read result if measurement is valid
if status.error_code == AlsErrorCode::NoError {
    let result: AlsResultValue = sensor.read_register()?;
    println!("ALS raw count: {}", result.raw_count);
} else {
    println!("ALS error: {:?}", status.error_code);
}
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
