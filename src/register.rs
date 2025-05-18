/// Register addresses for the APDS-9306
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    /// Main control register
    MainCtrl = 0x00,
    /// Measurement rate and resolution register
    MeasRate = 0x04,
    /// Gain register
    Gain = 0x05,
    /// Part ID register
    PartId = 0x06,
    /// Main status register
    MainStatus = 0x07,
    /// Clear ADC data LSB register
    ClearData0 = 0x0A,
    /// Clear ADC data middle byte register
    ClearData1 = 0x0B,
    /// Clear ADC data MSB register
    ClearData2 = 0x0C,
    /// ADC data LSB register
    Data0 = 0x0D,
    /// ADC data middle byte register
    Data1 = 0x0E,
    /// ADC data MSB register
    Data2 = 0x0F,
    /// Interrupt configuration register
    IntCfg = 0x19,
    /// Interrupt persistence register
    IntPersistence = 0x1A,
    /// Upper threshold LSB register
    ThresUp0 = 0x21,
    /// Upper threshold middle byte register
    ThresUp1 = 0x22,
    /// Upper threshold MSB register
    ThresUp2 = 0x23,
    /// Lower threshold LSB register
    ThresLow0 = 0x24,
    /// Lower threshold middle byte register
    ThresLow1 = 0x25,
    /// Lower threshold MSB register
    ThresLow2 = 0x26,
    /// Variance threshold register
    ThresVar = 0x27,
}

/// Bit positions in the MAIN_CTRL register
pub mod main_ctrl {
    /// Software reset bit
    pub const SW_RESET: u8 = 1 << 4;
    /// enable bit
    pub const EN: u8 = 1 << 1;
}

/// Bit positions in the MAIN_STATUS register
pub mod main_status {
    /// Power-on status bit
    pub const POWER_ON_STATUS: u8 = 1 << 5;
    /// interrupt status bit
    pub const INT_STATUS: u8 = 1 << 4;
    /// data status bit
    pub const DATA_STATUS: u8 = 1 << 3;
}

/// Bit positions in the INT_CFG register
pub mod int_cfg {
    /// interrupt source selection bits
    pub const INT_SEL_MASK: u8 = 0x30;
    /// interrupt source selection shift
    pub const INT_SEL_SHIFT: u8 = 4;
    /// variation interrupt mode bit
    pub const VAR_MODE: u8 = 1 << 3;
    /// interrupt enable bit
    pub const INT_EN: u8 = 1 << 2;
}

/// Bit positions in the INT_PERSISTENCE register
pub mod int_persistence {
    /// persistence bits
    pub const PERSIST_MASK: u8 = 0xF0;
    /// persistence shift
    pub const PERSIST_SHIFT: u8 = 4;
}
