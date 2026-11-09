#[derive(Debug, Clone, Copy)]
pub struct Bgr555(pub u16);

impl From<[u8; 2]> for Bgr555 {
    fn from(value: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(value))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb565(pub u16);

impl From<[u8; 2]> for Rgb565 {
    fn from(value: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(value))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb888 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Bgr555> for Rgb888 {
    #[inline]
    fn from(value: Bgr555) -> Self {
        /*
         * The NDS palette uses BGR555 for color encoding but we need RGB888
         * So, each individual color must be isolated and converted to RGB888
         */

        let c = value.0;

        let r5 = (c & 0x001F) as u8;
        let g5 = ((c >> 5) & 0x001F) as u8;
        let b5 = ((c >> 10) & 0x001F) as u8;

        Self {
            r: (r5 << 3) | (r5 >> 2),
            g: (g5 << 3) | (g5 >> 2),
            b: (b5 << 3) | (b5 >> 2),
        }
    }
}

impl From<Rgb565> for Rgb888 {
    #[inline]
    fn from(value: Rgb565) -> Self {
        /*
         * The 3DS icon usually uses RGB565 for color encoding, although others are also supported,
         * but we need RGB888
         * So, each individual color must be isolated and converted to RGB888
         */

        let c = value.0;

        let r5 = ((c >> 11) & 0x001F) as u8;
        let g6 = ((c >> 5) & 0x003F) as u8;
        let b5 = (c & 0x001F) as u8;

        Self {
            r: (r5 << 3) | (r5 >> 2),
            g: (g6 << 2) | (g6 >> 4),
            b: (b5 << 3) | (b5 >> 2),
        }
    }
}
