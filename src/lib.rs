//! # APDS-9306/APDS-9306-065 Async Driver
//!
//! An async embedded-hal driver for the APDS-9306/APDS-9306-065 Digital Ambient Light Sensor.
//!
//! This driver provides a comprehensive, trait-based interface for all chip features
//! and uses embedded-hal-async for all hardware transactions.
//!
//! ## Features
//!
//! - Async I2C communication
//! - Full support for all sensor features
//! - Configurable ALS (Ambient Light Sensor) settings
//! - Interrupt configuration and handling
//! - Power management
//!
//! ## Example
//!
//! ```rust,no_run
//! use apds9306::{Apds9306, Config, Resolution, MeasurementRate, Gain};
//! use embedded_hal_async::i2c::I2c;
//!
//! async fn example<I2C>(i2c: I2C) -> Result<(), apds9306::Error<I2C::Error>>
//! where
//!     I2C: I2c,
//! {
//!     // Create a new driver instance
//!     let mut sensor = Apds9306::new(i2c, Apds9306Type::Apds9306).await?;
//!
//!     // Reset the device
//!     sensor.reset().await?;
//!
//!     // Configure
//!     let config = Config {
//!         resolution: Resolution::Bits18,
//!         measurement_rate: MeasurementRate::Ms100,
//!         gain: Gain::Gain3,
//!     };
//!     sensor.configure(config).await?;
//!
//!     // Enable
//!     sensor.enable().await?;
//!
//!     // Wait for data to be ready
//!     while !sensor.is_data_ready().await? {
//!         // In a real application, you might want to use a timer
//!     }
//!
//!     // Read data
//!     let als_data = sensor.read_data().await?;
//!     println!("ALS data: {}", als_data);
//!
//!     Ok(())
//! }
//! ```

#![no_std]
#![warn(missing_docs)]

mod register;
mod types;

pub use register::Register;
pub use types::*;

use embedded_hal_async::i2c::I2c;

pub enum Apds9306Type {
    Apds9306,
    Apds9306_065,
}

/// I2C address for APDS-9306
pub const I2C_ADDR: u8 = 0x52;

/// Error type for the APDS-9306 driver
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2c(E),
    /// Invalid configuration
    InvalidConfig,
    /// Device not found or invalid ID
    DeviceNotFound,
    /// Operation timeout
    Timeout,
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2c(error)
    }
}

/// APDS-9306 driver
pub struct Apds9306<I2C> {
    /// I2C interface
    i2c: I2C,
    /// Current ALS configuration
    config: Config,
    /// Current interrupt configuration
    interrupt_config: InterruptConfig,
}

impl<I2C, E> Apds9306<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Creates a new APDS-9306 driver with default configuration and address
    pub async fn new(i2c: I2C, apds9306_type: Apds9306Type) -> Result<Self, Error<E>> {
        let mut driver = Self {
            i2c,
            config: Config::default(),
            interrupt_config: InterruptConfig::default(),
        };
        
        // verify device ID
        driver.verify_device_id(apds9306_type).await?;
        
        Ok(driver)
    }
    
    /// Verifies the device ID
    async fn verify_device_id(&mut self, apds9306_type: Apds9306Type) -> Result<(), Error<E>> {
        let part_id = self.read_register(Register::PartId).await?;
        
        // check if the part ID matches either APDS-9306 (0xB1) or APDS-9306-065 (0xB3)
        match apds9306_type {
            Apds9306Type::Apds9306 => {
                if part_id != 0xB1 {
                    return Err(Error::DeviceNotFound);
                }
            }
            Apds9306Type::Apds9306_065 => {
                if part_id != 0xB3 {
                    return Err(Error::DeviceNotFound);
                }
            }
        }
        
        Ok(())
    }
    
    /// Performs a software reset
    pub async fn reset(&mut self) -> Result<(), Error<E>> {
        // set SW_Reset bit (bit 4) in MAIN_CTRL register
        let value = register::main_ctrl::SW_RESET;
        self.write_register(Register::MainCtrl, value).await?;
        
        // wait for reset to complete (typical time not specified in datasheet)
        // may want to add a delay here
        
        Ok(())
    }
    
    /// Enables the sensor
    pub async fn enable(&mut self) -> Result<(), Error<E>> {
        // read current value
        let current = self.read_register(Register::MainCtrl).await?;
        
        // set ALS_EN bit (bit 1)
        let value = current | register::main_ctrl::EN;
        self.write_register(Register::MainCtrl, value).await?;
        
        Ok(())
    }
    
    /// Disables the sensor
    pub async fn disable(&mut self) -> Result<(), Error<E>> {
        // read current value
        let current = self.read_register(Register::MainCtrl).await?;
        
        // clear ALS_EN bit (bit 1)
        let value = current & !register::main_ctrl::EN;
        self.write_register(Register::MainCtrl, value).await?;
        
        Ok(())
    }
    
    /// Configures the sensor
    pub async fn configure(&mut self, config: Config) -> Result<(), Error<E>> {
        // update stored configuration
        self.config = config;
        
        // configure MEAS_RATE register
        let meas_rate_value = ((config.resolution as u8) << 4) | (config.measurement_rate as u8);
        self.write_register(Register::MeasRate, meas_rate_value).await?;
        
        // configure GAIN register
        self.write_register(Register::Gain, config.gain as u8).await?;
        
        Ok(())
    }
    
    /// Configures the interrupt system
    pub async fn configure_interrupt(&mut self, config: InterruptConfig) -> Result<(), Error<E>> {
        // update stored configuration
        self.interrupt_config = config;
        
        // configure INT_CFG register
        let int_cfg_value = ((config.source as u8) << register::int_cfg::INT_SEL_SHIFT) | 
                           ((config.mode as u8) << 3) | 
                           ((config.enabled as u8) << 2);
        self.write_register(Register::IntCfg, int_cfg_value).await?;
        
        // configure INT_PERSISTENCE register
        let persistence = if config.persistence > 15 { 15 } else { config.persistence };
        let int_persistence_value = persistence << register::int_persistence::PERSIST_SHIFT;
        self.write_register(Register::IntPersistence, int_persistence_value).await?;
        
        // configure upper threshold
        let upper = config.upper_threshold & 0xFFFFF; // Ensure 20-bit max
        self.write_register(Register::ThresUp0, (upper & 0xFF) as u8).await?;
        self.write_register(Register::ThresUp1, ((upper >> 8) & 0xFF) as u8).await?;
        self.write_register(Register::ThresUp2, ((upper >> 16) & 0x0F) as u8).await?;
        
        // configure lower threshold
        let lower = config.lower_threshold & 0xFFFFF; // Ensure 20-bit max
        self.write_register(Register::ThresLow0, (lower & 0xFF) as u8).await?;
        self.write_register(Register::ThresLow1, ((lower >> 8) & 0xFF) as u8).await?;
        self.write_register(Register::ThresLow2, ((lower >> 16) & 0x0F) as u8).await?;
        
        // configure variance threshold
        self.write_register(Register::ThresVar, config.variance_threshold as u8).await?;
        
        Ok(())
    }
    
    /// Reads the ALS data
    pub async fn read_data(&mut self) -> Result<u32, Error<E>> {
        let mut buffer = [0u8; 3];
        self.read_registers(Register::Data0, &mut buffer).await?;
        
        // combine the 3 bytes into a 20-bit value
        let als_data = (buffer[0] as u32) | ((buffer[1] as u32) << 8) | ((buffer[2] as u32 & 0x0F) << 16);
        
        Ok(als_data)
    }
    
    /// Reads the Clear data
    pub async fn read_clear_data(&mut self) -> Result<u32, Error<E>> {
        let mut buffer = [0u8; 3];
        self.read_registers(Register::ClearData0, &mut buffer).await?;
        
        // combine the 3 bytes into a 20-bit value
        let clear_data = (buffer[0] as u32) | ((buffer[1] as u32) << 8) | ((buffer[2] as u32 & 0x0F) << 16);
        
        Ok(clear_data)
    }
    
    /// Reads both ALS and Clear data in a single operation
    pub async fn read_measurement_data(&mut self) -> Result<MeasurementData, Error<E>> {
        Ok(MeasurementData {
            als: self.read_data().await?,
            clear: self.read_clear_data().await?,
        })
    }
    
    /// Reads the status register
    pub async fn read_status(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::MainStatus).await
    }
    
    /// Reads and parses the status register into a Status struct
    pub async fn get_status(&mut self) -> Result<Status, Error<E>> {
        let status_reg = self.read_status().await?;
        
        Ok(Status {
            power_on: (status_reg & register::main_status::POWER_ON_STATUS) != 0,
            interrupt: (status_reg & register::main_status::INT_STATUS) != 0,
            data_ready: (status_reg & register::main_status::DATA_STATUS) != 0,
        })
    }
    
    /// Checks if power-on status is set
    pub async fn is_power_on_status(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_status().await?;
        Ok((status & register::main_status::POWER_ON_STATUS) != 0)
    }
    
    /// Checks if ALS interrupt is triggered
    pub async fn is_interrupt(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_status().await?;
        Ok((status & register::main_status::INT_STATUS) != 0)
    }
    
    /// Checks if new ALS data is available
    pub async fn is_data_ready(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_status().await?;
        Ok((status & register::main_status::DATA_STATUS) != 0)
    }
    
    /// Gets the current ALS configuration
    pub fn get_config(&self) -> Config {
        self.config
    }
    
    /// Gets the current interrupt configuration
    pub fn get_interrupt_config(&self) -> InterruptConfig {
        self.interrupt_config
    }
    
    /// Reads a single register
    async fn read_register(&mut self, register: Register) -> Result<u8, Error<E>> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(I2C_ADDR, &[register as u8], &mut buffer).await?;
        Ok(buffer[0])
    }
    
    /// Writes a value to a register
    async fn write_register(&mut self, register: Register, value: u8) -> Result<(), Error<E>> {
        self.i2c.write(I2C_ADDR, &[register as u8, value]).await?;
        Ok(())
    }
    
    /// Reads multiple consecutive registers
    async fn read_registers(&mut self, start_register: Register, buffer: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c.write_read(I2C_ADDR, &[start_register as u8], buffer).await?;
        Ok(())
    }
    
    /// Releases the I2C interface
    pub fn release(self) -> I2C {
        self.i2c
    }
}
