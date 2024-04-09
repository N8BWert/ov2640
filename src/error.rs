//!
//! Error from operating the OV2640 Module
//! 

pub enum OV2640Error<I2CErr, SPIErr> {
    CannotSetImageSizeOnNonJPEG,
    // buffer is too small
    InvalidBufferSize,
    NoI2cPeripheral,
    I2CError(I2CErr),
    NoSpiPeripheral,
    SpiError(SPIErr),
}