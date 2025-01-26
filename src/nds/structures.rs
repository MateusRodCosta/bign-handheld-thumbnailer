use image::{ImageBuffer, Rgba};

use super::ParsingError;

#[derive(Debug)]
pub struct PaletteColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl PaletteColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> PaletteColor {
        PaletteColor { r, g, b, a }
    }
}

#[derive(Debug)]
pub struct NDSBannerDetails {
    _icon_version: NDSIconVersion,
    icon: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl NDSBannerDetails {
    pub fn new(
        icon_version: NDSIconVersion,
        icon: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> NDSBannerDetails {
        NDSBannerDetails {
            _icon_version: icon_version,
            icon,
        }
    }

    pub fn get_icon(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.icon.clone()
    }
}

/// The NDS icon versions map to this:
///
/// 0001h = Original,
/// 0002h = With Chinese Title,
/// 0003h = With Chinese+Korean Titles,
/// 0103h = With Chinese+Korean Titles and animated DSi icon
///
/// Do note that the animated DSi icon is not supported by this thumbnailer

#[derive(Debug)]
pub enum NDSIconVersion {
    V1,
    V2,
    V3,
    DSi,
}

impl TryFrom<u16> for NDSIconVersion {
    type Error = ParsingError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(NDSIconVersion::V1),
            0x0002 => Ok(NDSIconVersion::V2),
            0x0003 => Ok(NDSIconVersion::V3),
            0x0103 => Ok(NDSIconVersion::DSi),
            _ => Err(Self::Error::UnknownOrInvalidNDSIconVersion(value)),
        }
    }
}
