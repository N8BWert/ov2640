//!
//! Driver for the OV2640 ArduCam Module
//! 

#![no_std]

pub mod config;
pub use config::{ImageFormat, Resolution, LightMode, Saturation, Brightness, Contrast, SpecialEffect, Configuration, ConfigurationBuilder};

pub mod error;
pub use error::OV2640Error;

mod register;
use register::*;

use embedded_hal::{i2c::{I2c, SevenBitAddress}, spi::SpiDevice, delay::DelayNs};

/// Maximum Frame Buffer Size (384KBytes)
pub const MAX_FIFO_SIZE: usize = 0x5FFFF;
/// Address of the OV2640
pub const I2C_ADDRESS: u8 = 0x60;

/// Clear FIFO MASK
pub const FIFO_CLEAR_MASK: u8 = 0x00;
/// Begin Capture FIFO Mask
pub const FIFO_START_MASK: u8 = 0x00;
/// Capture Complete Mask
pub const CAPTURE_COMPLETE_MASK: u8 = 0x08;
/// Allow FIFO to be read at once
pub const FIFO_BURST: u8 = 0x3C;

pub struct OV2640<I2C, SPI> {
    // Configuration
    configuration: Configuration,
    // I2C Peripheral
    i2c: Option<I2C>,
    // SPI Peripheral
    spi: Option<SPI>,
}

impl<I2C, SPI, I2CErr, SPIErr> OV2640<I2C, SPI> where
    I2C: I2c<SevenBitAddress, Error=I2CErr>,
    SPI: SpiDevice<u8, Error=SPIErr> {
    /// Initialize a new OV2640 Driver
    pub fn new(i2c: Option<I2C>, spi: Option<SPI>) -> Self {
        Self {
            configuration: ConfigurationBuilder::default().build(),
            i2c,
            spi,
        }
    }

    /// Initialize a new OV2640 Driver with given configuration
    pub fn with_configuration(
        configuration: Configuration, i2c: Option<I2C>, spi: Option<SPI>
    ) -> Self {
        Self {
            configuration,
            i2c,
            spi,
        }
    }

    /// Check that I2C is correctly connected to the OV2640 Module
    pub fn i2c_connected(&mut self) -> Result<bool, OV2640Error<I2CErr, SPIErr>> {
        self.write_spi(TEST_REGISTER, 0x52)?;
        let result = self.read_spi(TEST_REGISTER)?;
        Ok(result == 0x52)
    }

    /// Check that SPI is correctly connected to the OV2640 Module
    pub fn spi_connected(&mut self) -> Result<bool, OV2640Error<I2CErr, SPIErr>> {
        self.write_register(0xFF, 0x01)?;
        let high = self.read_register(CHIP_ID_HIGH)?;
        let low = self.read_register(CHIP_ID_LOW)?;
        // Check a valid chip ID was found
        Ok(
            low == 0x26 &&
            (high == 0x41 || high == 0x42)
        )
    }

    /// Initialize the OV2640 Driver with its configuration
    pub fn init(&mut self, delay: &mut dyn DelayNs) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.set_image_format(self.configuration.image_format, delay)?;
        self.set_resolution(self.configuration.resolution)?;
        self.set_light_mode(self.configuration.light_mode)?;
        self.set_saturation(self.configuration.saturation)?;
        self.set_brightness(self.configuration.brightness)?;
        self.set_contrast(self.configuration.contrast)?;
        self.set_special_effect(self.configuration.special_effect)
    }

    /// Set the configuration of the OV2640 Driver
    pub fn set_configuration(
        &mut self, configuration: Configuration, delay: &mut dyn DelayNs
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.configuration = configuration;
        self.init(delay)
    }

    /// Set the image format for the OV2640 Module
    pub fn set_image_format(
        &mut self, image_format: ImageFormat, delay: &mut dyn DelayNs
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.write_register(0xFF, 0x01)?;
        self.write_register(0x12, 0x80)?;
        delay.delay_ms(100);

        match image_format {
            ImageFormat::JPEG => {
                self.write_registers(&JPEG_INIT_REGISTER)?;
                self.write_registers(&YUV422_REGISTERS)?;
                self.write_registers(&JPEG_REGISTERS)?;
                self.write_register(0xFF, 0x01)?;
                self.write_register(0x15, 0x00)?;
                self.set_resolution(self.configuration.resolution)?;
            },
            ImageFormat::QVGA => self.write_registers(&QVGA_REGISTERS)?,
        }
        self.configuration.image_format = image_format;
        Ok(())
    }

    /// Set the resolution of the OV2640 Module
    pub fn set_resolution(
        &mut self, resolution: Resolution
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        if self.configuration.image_format != ImageFormat::JPEG {
            return Err(OV2640Error::CannotSetImageSizeOnNonJPEG);
        }

        match resolution {
            Resolution::R160x120 => self.write_registers(&JPEG_160x120_REGISTERS)?,
            Resolution::R176x144 => self.write_registers(&JPEG_176x144_REGISTERS)?,
            Resolution::R320x240 => self.write_registers(&JPEG_320x240_REGISTERS)?,
            Resolution::R352x288 => self.write_registers(&JPEG_352x288_REGISTERS)?,
            Resolution::R640x480 => self.write_registers(&JPEG_640x480_REGISTERS)?,
            Resolution::R800x600 => self.write_registers(&JPEG_800x600_REGISTERS)?,
            Resolution::R1024x768 => self.write_registers(&JPEG_1024x768_REGISTERS)?,
            Resolution::R1280x1024 => self.write_registers(&JPEG_1280x1024_REGISTERS)?,
            Resolution::R1600x1200 => self.write_registers(&JPEG_1600x1200_REGISTERS)?,
        }
        self.configuration.resolution = resolution;
        Ok(())
    }

    /// Set the light mode of the OV2640 Module
    pub fn set_light_mode(
        &mut self, light_mode: LightMode,
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.write_register(0xFF, 0x00)?;
        match light_mode {
            LightMode::Auto => self.write_register(0xC7, 0x00)?,
            LightMode::Sunny => {
                self.write_register(0xC7, 0x40)?;
                self.write_register(0xCC, 0x5E)?;
                self.write_register(0xCD, 0x41)?;
                self.write_register(0xCE, 0x54)?;
            },
            LightMode::Cloudy => {
                self.write_register(0xC7, 0x40)?;
                self.write_register(0xCC, 0x65)?;
                self.write_register(0xCD, 0x41)?;
                self.write_register(0xCE, 0x4F)?;
            },
            LightMode::Office => {
                self.write_register(0xC7, 0x40)?;
                self.write_register(0xCC, 0x52)?;
                self.write_register(0xCD, 0x41)?;
                self.write_register(0xCE, 0x6)?;
            },
            LightMode::Home => {
                self.write_register(0xC7, 0x40)?;
                self.write_register(0xCC, 0x42)?;
                self.write_register(0xCD, 0x3F)?;
                self.write_register(0xCE, 0x71)?;
            },
        }
        self.configuration.light_mode = light_mode;
        Ok(())
    }

    /// Set the saturation of the OV2640 Module
    pub fn set_saturation(
        &mut self, saturation: Saturation
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.write_register(0xFF, 0x00)?;
        self.write_register(0x7C, 0x00)?;
        self.write_register(0x7D, 0x02)?;
        self.write_register(0x7C, 0x04)?;

        match saturation {
            Saturation::Saturation0 => {
                self.write_register(0x7D, 0x68)?;
                self.write_register(0x7D, 0x68)?;
            },
            Saturation::Saturation1 => {
                self.write_register(0x7D, 0x58)?;
                self.write_register(0x7D, 0x58)?;
            },
            Saturation::Saturation2 => {
                self.write_register(0x7D, 0x48)?;
                self.write_register(0x7D, 0x48)?;
            },
            Saturation::Saturation3 => {
                self.write_register(0x7D, 0x38)?;
                self.write_register(0x7D, 0x38)?;
            },
            Saturation::Saturation4 => {
                self.write_register(0x7D, 0x28)?;
                self.write_register(0x7D, 0x28)?;
            }
        }
        self.configuration.saturation = saturation;
        Ok(())
    }

    /// Set the brightness of the OV2640 Module
    pub fn set_brightness(
        &mut self, brightness: Brightness
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.write_register(0xFF, 0x00)?;
        self.write_register(0x7C, 0x00)?;
        self.write_register(0x7D, 0x04)?;
        self.write_register(0x7C, 0x09)?;

        match brightness {
            Brightness::Brightness0 => self.write_register(0x7D, 0x40)?,
            Brightness::Brightness1 => self.write_register(0x7D, 0x30)?,
            Brightness::Brightness2 => self.write_register(0x7D, 0x20)?,
            Brightness::Brightness3 => self.write_register(0x7D, 0x10)?,
            Brightness::Brightness4 => self.write_register(0x7D, 0x00)?,
        }

        self.write_register(0x7D, 0x00)?;
        self.configuration.brightness = brightness;
        Ok(())
    }

    pub fn set_contrast(
        &mut self, contrast: Contrast
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.write_register(0xFF, 0x00)?;
        self.write_register(0x7C, 0x00)?;
        self.write_register(0x7D, 0x04)?;
        self.write_register(0x7C, 0x07)?;
        self.write_register(0x7D, 0x20)?;

        match contrast {
            Contrast::Contrast0 => {
                self.write_register(0x7D, 0x28)?;
                self.write_register(0x7D, 0x0C)?;
            },
            Contrast::Contrast1 => {
                self.write_register(0x7D, 0x24)?;
                self.write_register(0x7D, 0x16)?;
            },
            Contrast::Contrast2 => {
                self.write_register(0x7D, 0x20)?;
                self.write_register(0x7D, 0x20)?;
            },
            Contrast::Contrast3 => {
                self.write_register(0x7D, 0x20)?;
                self.write_register(0x7D, 0x2A)?;
            },
            Contrast::Contrast4 => {
                self.write_register(0x7D, 0x18)?;
                self.write_register(0x7D, 0x34)?;
            }
        }

        self.write_register(0x7D, 0x06)?;
        self.configuration.contrast = contrast;
        Ok(())
    }

    /// Set the special effect used by the OV2640 Module
    pub fn set_special_effect(
        &mut self, special_effect: SpecialEffect
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.write_register(0xFF, 0x00)?;
        self.write_register(0x7C, 0x00)?;

        match special_effect {
            SpecialEffect::Antique => {
                self.write_register(0x7D, 0x18)?;
                self.write_register(0x7C, 0x05)?;
                self.write_register(0x7D, 0x40)?;
                self.write_register(0x7D, 0xA6)?;
            },
            SpecialEffect::Bluish => {
                self.write_register(0x7D, 0x18)?;
                self.write_register(0x7C, 0x05)?;
                self.write_register(0x7D, 0xA0)?;
                self.write_register(0x7D, 0x40)?;
            },
            SpecialEffect::Greenish => {
                self.write_register(0x7D, 0x18)?;
                self.write_register(0x7C, 0x05)?;
                self.write_register(0x7D, 0x40)?;
                self.write_register(0x7D, 0x40)?;
            },
            SpecialEffect::Reddish => {
                self.write_register(0x7D, 0x18)?;
                self.write_register(0x7C, 0x05)?;
                self.write_register(0x7D, 0x40)?;
                self.write_register(0x7D, 0xC0)?;
            },
            SpecialEffect::BlackWhite => {
                self.write_register(0x7D, 0x18)?;
                self.write_register(0x7C, 0x05)?;
                self.write_register(0x7D, 0x80)?;
                self.write_register(0x7D, 0x80)?;
            },
            SpecialEffect::Negative => {
                self.write_register(0x7D, 0x40)?;
                self.write_register(0x7C, 0x05)?;
                self.write_register(0x7D, 0x80)?;
                self.write_register(0x7D, 0x80)?;
            },
            SpecialEffect::BlackWhiteNegative => {
                self.write_register(0x7D, 0x58)?;
                self.write_register(0x7C, 0x05)?;
                self.write_register(0x7D, 0x80)?;
                self.write_register(0x7D, 0x80)?;
            },
            SpecialEffect::Normal => {
                self.write_register(0x7D, 0x00)?;
                self.write_register(0x7C, 0x05)?;
                self.write_register(0x7D, 0x80)?;
                self.write_register(0x7D, 0x80)?;
            }
        }

        self.configuration.special_effect = special_effect;
        Ok(())
    }

    /// Flush the OV2640's FIFO
    pub fn flush_fifo(&mut self) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.write_spi(FIFO, FIFO_CLEAR_MASK)
    }

    /// Start capturing into the FIFO
    pub fn start_capture(&mut self) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        self.write_spi(FIFO, FIFO_CLEAR_MASK)?;
        self.write_spi(FIFO, FIFO_START_MASK)
    }

    /// Check whether the capture is complete
    pub fn is_capture_done(&mut self) -> Result<bool, OV2640Error<I2CErr, SPIErr>> {
        Ok(self.read_spi(TRIGGER)? & CAPTURE_COMPLETE_MASK != 0)
    }

    /// Get the length of the image in the FIFO
    pub fn image_size(&mut self) -> Result<usize, OV2640Error<I2CErr, SPIErr>> {
        let len1 = self.read_spi(FIFO_SIZE_1)?;
        let len2 = self.read_spi(FIFO_SIZE_2)?;
        let len3 = self.read_spi(FIFO_SIZE_3)?;

        Ok(u32::from_be_bytes([0x00, len3, len2, len1]) as usize)
    }

    /// Read the captured image into the provided buffer, returning the image
    /// length in bytes
    pub fn read_image(
        &mut self, buffer: &mut [u8]
    ) -> Result<usize, OV2640Error<I2CErr, SPIErr>> {
        let image_size = self.image_size()?;
        if buffer.len() < image_size {
            return Err(OV2640Error::InvalidBufferSize)?;
        }

        if let Some(spi) = self.spi.as_mut() {
            spi.write(&[FIFO_BURST]).map_err(OV2640Error::SpiError)?;
            spi.transfer_in_place(buffer).map_err(OV2640Error::SpiError)?;
            Ok(image_size)
        } else {
            Err(OV2640Error::NoSpiPeripheral)
        }
    }

    /// Take the SPI Peripheral from the device
    pub fn take_spi(&mut self) -> Option<SPI> {
        self.spi.take()
    }

    /// Take the I2C Peripheral from the device
    pub fn take_i2c(&mut self) -> Option<I2C> {
        self.i2c.take()
    }

    /// Write to an SPI register
    fn write_spi(
        &mut self, address: u8, value: u8
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        if let Some(spi) = self.spi.as_mut() {
            spi.write(&[address | 0x80, value]).map_err(OV2640Error::SpiError)
        } else {
            Err(OV2640Error::NoSpiPeripheral)
        }
    }

    /// Read from an SPI register
    fn read_spi(
        &mut self, address: u8,
    ) -> Result<u8, OV2640Error<I2CErr, SPIErr>> {
        if let Some(spi) = self.spi.as_mut() {
            let mut buffer = [address];
            spi.transfer_in_place(&mut buffer).map_err(OV2640Error::SpiError)?;
            Ok(buffer[0])
        } else {
            Err(OV2640Error::NoSpiPeripheral)
        }
    }

    /// Write to a singular register via I2C
    fn write_register(
        &mut self, register: u8, value: u8
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        if let Some(i2c) = self.i2c.as_mut() {
            i2c.write(I2C_ADDRESS, &[register, value])
                .map_err(OV2640Error::I2CError)
        } else {
            Err(OV2640Error::NoI2cPeripheral)
        }
    }

    /// Write to a set of registers via I2C
    fn write_registers(
        &mut self, registers: &[[u8; 2]]
    ) -> Result<(), OV2640Error<I2CErr, SPIErr>> {
        for register in registers {
            self.write_register(register[0], register[1])?;
        }
        Ok(())
    }

    /// Read the value from a register via I2C
    fn read_register(
        &mut self, register: u8
    ) -> Result<u8, OV2640Error<I2CErr, SPIErr>> {
        if let Some(i2c) = self.i2c.as_mut() {
            let mut buffer = [0u8];
            i2c.write_read(I2C_ADDRESS, &[register], &mut buffer)
                .map_err(OV2640Error::I2CError)?;
            Ok(buffer[0])
        } else {
            Err(OV2640Error::NoI2cPeripheral)
        }
    }
}