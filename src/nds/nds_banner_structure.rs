use gdk_pixbuf::Pixbuf;

use super::UnknownOrInvalidNDSIconVersion;

#[derive(Debug)]
pub struct PaletteColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl PaletteColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> PaletteColor {
        PaletteColor { r, g, b, a }
    }

    pub fn get_r(&self) -> u8 {
        self.r
    }

    pub fn get_g(&self) -> u8 {
        self.g
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }
}

#[derive(Debug)]
pub struct NDSBannerDetails {
    _icon_version: NDSIconVersion,
    icon: Pixbuf,
}

impl NDSBannerDetails {
    pub fn new(icon_version: NDSIconVersion, icon: Pixbuf) -> NDSBannerDetails {
        NDSBannerDetails {
            _icon_version: icon_version,
            icon,
        }
    }

    pub fn _get_icon_version(&self) -> &NDSIconVersion {
        &self._icon_version
    }

    pub fn get_icon(&self) -> &Pixbuf {
        &self.icon
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
    type Error = UnknownOrInvalidNDSIconVersion;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(NDSIconVersion::V1),
            0x0002 => Ok(NDSIconVersion::V2),
            0x0003 => Ok(NDSIconVersion::V3),
            0x0103 => Ok(NDSIconVersion::DSi),
            _ => Err(Self::Error { 0: value }),
        }
    }
}
