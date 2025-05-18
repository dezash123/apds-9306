use core::fmt;

/// Resolution/bit width settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Resolution {
    /// 20-bit resolution, 400ms conversion time
    Bits20 = 0b000,
    /// 19-bit resolution, 200ms conversion time
    Bits19 = 0b001,
    /// 18-bit resolution, 100ms conversion time (default)
    Bits18 = 0b010,
    /// 17-bit resolution, 50ms conversion time
    Bits17 = 0b011,
    /// 16-bit resolution, 25ms conversion time
    Bits16 = 0b100,
    /// 13-bit resolution, 3.125ms conversion time
    Bits13 = 0b101,
}

impl Default for Resolution {
    fn default() -> Self {
        Self::Bits18
    }
}

/// Measurement rate settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MeasurementRate {
    /// 25ms measurement rate
    Ms25 = 0b000,
    /// 50ms measurement rate
    Ms50 = 0b001,
    /// 100ms measurement rate (default)
    Ms100 = 0b010,
    /// 200ms measurement rate
    Ms200 = 0b011,
    /// 500ms measurement rate
    Ms500 = 0b100,
    /// 1000ms measurement rate
    Ms1000 = 0b101,
    /// 2000ms measurement rate
    Ms2000 = 0b110,
}

impl Default for MeasurementRate {
    fn default() -> Self {
        Self::Ms100
    }
}

/// Gain settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Gain {
    /// Gain 1
    Gain1 = 0b000,
    /// Gain 3 (default)
    Gain3 = 0b001,
    /// Gain 6
    Gain6 = 0b010,
    /// Gain 9
    Gain9 = 0b011,
    /// Gain 18
    Gain18 = 0b100,
}

impl Default for Gain {
    fn default() -> Self {
        Self::Gain3
    }
}

/// Interrupt source selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InterruptSource {
    /// Clear channel
    ClearChannel = 0b00,
    /// ALS channel (default)
    AlsChannel = 0b01,
}

impl Default for InterruptSource {
    fn default() -> Self {
        Self::AlsChannel
    }
}

/// Interrupt mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InterruptMode {
    /// Threshold interrupt mode (default)
    Threshold = 0,
    /// Variation interrupt mode
    Variation = 1,
}

impl Default for InterruptMode {
    fn default() -> Self {
        Self::Threshold
    }
}

/// Variance threshold settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VarianceThreshold {
    /// Result varies by 8 counts compared to previous result
    Counts8 = 0b000,
    /// Result varies by 16 counts compared to previous result
    Counts16 = 0b001,
    /// Result varies by 32 counts compared to previous result
    Counts32 = 0b010,
    /// Result varies by 64 counts compared to previous result
    Counts64 = 0b011,
    /// Result varies by 128 counts compared to previous result
    Counts128 = 0b100,
    /// Result varies by 256 counts compared to previous result
    Counts256 = 0b101,
    /// Result varies by 512 counts compared to previous result
    Counts512 = 0b110,
    /// Result varies by 1024 counts compared to previous result
    Counts1024 = 0b111,
}

impl Default for VarianceThreshold {
    fn default() -> Self {
        Self::Counts8
    }
}

/// Configuration for measurement
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Resolution/bit width
    pub resolution: Resolution,
    /// Measurement rate
    pub measurement_rate: MeasurementRate,
    /// Gain
    pub gain: Gain,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resolution: Resolution::default(),
            measurement_rate: MeasurementRate::default(),
            gain: Gain::default(),
        }
    }
}

/// Configuration for interrupts
#[derive(Debug, Clone, Copy)]
pub struct InterruptConfig {
    /// Interrupt source
    pub source: InterruptSource,
    /// Interrupt mode
    pub mode: InterruptMode,
    /// Interrupt enabled
    pub enabled: bool,
    /// Number of consecutive measurements outside threshold before interrupt (0-15)
    pub persistence: u8,
    /// Upper threshold for interrupt (20-bit value)
    pub upper_threshold: u32,
    /// Lower threshold for interrupt (20-bit value)
    pub lower_threshold: u32,
    /// Variance threshold for variation mode
    pub variance_threshold: VarianceThreshold,
}

impl Default for InterruptConfig {
    fn default() -> Self {
        Self {
            source: InterruptSource::default(),
            mode: InterruptMode::default(),
            enabled: false,
            persistence: 0,
            upper_threshold: 0x0FFFFF, // Maximum 20-bit value
            lower_threshold: 0,
            variance_threshold: VarianceThreshold::default(),
        }
    }
}

/// Status information from the APDS-9306
#[derive(Debug, Clone, Copy)]
pub struct Status {
    /// Power-on status
    pub power_on: bool,
    /// Interrupt status
    pub interrupt: bool,
    /// Data ready status
    pub data_ready: bool,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Status {{ power_on: {}, interrupt: {}, data_ready: {} }}",
            self.power_on, self.interrupt, self.data_ready
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Status {{ power_on: {}, interrupt: {}, data_ready: {} }}",
            self.power_on, self.interrupt, self.data_ready
        );
    }
}

/// Measurement data from the APDS-9306
#[derive(Debug, Clone, Copy)]
pub struct MeasurementData {
    /// ALS channel data
    pub als: u32,
    /// Clear channel data
    pub clear: u32,
}

impl fmt::Display for MeasurementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MeasurementData {{ als: {}, clear: {} }}",
            self.als, self.clear
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for MeasurementData {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "MeasurementData {{ als: {}, clear: {} }}",
            self.als, self.clear
        );
    }
}
