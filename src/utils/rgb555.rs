pub struct Rgb555(u8, u8, u8);

impl TryFrom<u16> for Rgb555 {
    type Error = Box<dyn std::error::Error>;

    fn try_from(color_bytes: u16) -> Result<Rgb555, Self::Error> {
        /*
         * The NDS palette uses RGB555 for color encoding but we need RGB888
         * So, each individual color must be isolated and converted to RGB888
         */

        let r = u8::try_from((color_bytes & 0x001F) << 3)?;
        let g = u8::try_from((color_bytes & 0x03E0) >> 2)?;
        let b = u8::try_from((color_bytes & 0x7C00) >> 7)?;

        Ok(Rgb555(r, g, b))
    }
}

impl Rgb555 {
    pub fn r(&self) -> u8 {
        self.0
    }
    pub fn g(&self) -> u8 {
        self.1
    }
    pub fn b(&self) -> u8 {
        self.2
    }
}
