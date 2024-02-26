use gdk_pixbuf::Pixbuf;

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
    icon_version: NDSIconVersion,
    icon: Pixbuf,
}

impl NDSBannerDetails {
    pub fn new(icon_version: NDSIconVersion, icon: Pixbuf) -> NDSBannerDetails {
        NDSBannerDetails { icon_version, icon }
    }

    pub fn get_icon_version(&self) -> &NDSIconVersion {
        &self.icon_version
    }

    pub fn get_icon(&self) -> &Pixbuf {
        &self.icon
    }
}

#[derive(Debug)]
pub enum NDSIconVersion {
    V1,
    V2,
    V3,
    DSi,
}
