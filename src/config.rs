//!
//! Configuration Options for the OV2640 Camera Module
//!

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    JPEG,
    QVGA,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolution {
    R160x120,
    R176x144,
    R320x240,
    R352x288,
    R640x480,
    R800x600,
    R1024x768,
    R1280x1024,
    R1600x1200,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightMode {
    Auto,
    Sunny,
    Cloudy,
    Office,
    Home,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saturation {
    Saturation0,
    Saturation1,
    Saturation2,
    Saturation3,
    Saturation4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brightness {
    Brightness0,
    Brightness1,
    Brightness2,
    Brightness3,
    Brightness4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contrast {
    Contrast0,
    Contrast1,
    Contrast2,
    Contrast3,
    Contrast4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialEffect {
    Normal,
    Antique,
    Bluish,
    Greenish,
    Reddish,
    BlackWhite,
    Negative,
    BlackWhiteNegative,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Configuration {
    pub image_format: ImageFormat,
    pub resolution: Resolution,
    pub light_mode: LightMode,
    pub saturation: Saturation,
    pub brightness: Brightness,
    pub contrast: Contrast,
    pub special_effect: SpecialEffect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConfigurationBuilder {
    image_format: Option<ImageFormat>,
    resolution: Option<Resolution>,
    light_mode: Option<LightMode>,
    saturation: Option<Saturation>,
    brightness: Option<Brightness>,
    contrast: Option<Contrast>,
    special_effect: Option<SpecialEffect>,
}

impl ConfigurationBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn image_format(mut self, image_format: ImageFormat) -> Self {
        self.image_format = Some(image_format);
        self
    }

    pub fn resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = Some(resolution);
        self
    }

    pub fn light_mode(mut self, light_mode: LightMode) -> Self {
        self.light_mode = Some(light_mode);
        self
    }

    pub fn saturation(mut self, saturation: Saturation) -> Self {
        self.saturation = Some(saturation);
        self
    }

    pub fn brightness(mut self, brightness: Brightness) -> Self {
        self.brightness = Some(brightness);
        self
    }

    pub fn contrast(mut self, contrast: Contrast) -> Self {
        self.contrast = Some(contrast);
        self
    }

    pub fn special_effect(mut self, special_effect: SpecialEffect) -> Self {
        self.special_effect = Some(special_effect);
        self
    }

    pub fn build(&self) -> Configuration {
        let image_format = match self.image_format {
            Some(image_format) => image_format,
            None => ImageFormat::JPEG,
        };

        let resolution = match self.resolution {
            Some(resolution) => resolution,
            None => Resolution::R1024x768,
        };

        let light_mode = match self.light_mode {
            Some(light_mode) => light_mode,
            None => LightMode::Auto,
        };

        let saturation = match self.saturation {
            Some(saturation) => saturation,
            None => Saturation::Saturation0,
        };

        let brightness = match self.brightness {
            Some(brightness) => brightness,
            None => Brightness::Brightness0,
        };

        let contrast = match self.contrast {
            Some(contrast) => contrast,
            None => Contrast::Contrast0,
        };

        let special_effect = match self.special_effect {
            Some(special_effect) => special_effect,
            None => SpecialEffect::Normal,
        };

        Configuration {
            image_format,
            resolution,
            light_mode,
            saturation,
            brightness,
            contrast,
            special_effect,
        }
    }
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        Self {
            image_format: None,
            resolution: None,
            light_mode: None,
            saturation: None,
            brightness: None,
            contrast: None,
            special_effect: None,
        }
    }
}