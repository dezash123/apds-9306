# APDS-9306 Async Rust Driver

An async embedded-hal driver for the APDS-9306/APDS-9306-065 Digital Ambient Light Sensor.

## Features

- **Fully Async**: All hardware transactions use embedded-hal-async
- **Complete Feature Support**: Implements all features described in the datasheet
- **Configurable**: Comprehensive configuration options for all sensor parameters
- **Error Handling**: Robust error handling with Result types
- **Documentation**: Thoroughly documented API with examples

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
apds9306 = "0.1.0"
embedded-hal = "1.0.0"
embedded-hal-async = "1.0.0"
```

## Example

```rust
use apds9306::{Apds9306, Config, Resolution, MeasurementRate, Gain};
use embedded_hal_async::i2c::I2c;

async fn example<I2C>(i2c: I2C) -> Result<(), apds9306::Error<I2C::Error>>
where
    I2C: I2c,
{
    // Create a new driver instance
    let mut sensor = Apds9306::new(i2c, Apds9306Type::Apds9306).await?;

    // Reset the device
    sensor.reset().await?;

    // Configure
    let config = Config {
        resolution: Resolution::Bits18,
        measurement_rate: MeasurementRate::Ms100,
        gain: Gain::Gain3,
    };
    sensor.configure(config).await?;

    // Enable
    sensor.enable().await?;

    // Wait for data to be ready
    while !sensor.is_data_ready().await? {
        // In a real application, you might want to use a timer
    }

    // Read data
    let als_data = sensor.read_data().await?;
    println!("ALS data: {}", als_data);

    Ok(())
}
```

## API Documentation

For detailed API documentation, please run:

```
cargo doc --open
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
