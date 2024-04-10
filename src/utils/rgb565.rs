pub struct Rgb565(u8, u8, u8);

impl TryFrom<u16> for Rgb565 {
    type Error = Box<dyn std::error::Error>;

    fn try_from(color_bytes: u16) -> Result<Rgb565, Self::Error> {
        /*
         * The 3DS icon usually uses BGR555 for color encoding, although others are also supported,
         * but we need RGB888
         * So, each individual color must be isolated and converted to RGB888
         */

        // Conversion code borrowed from
        // https://learn.microsoft.com/en-us/windows/win32/directshow/working-with-16-bit-rgb

        let red_value = u8::try_from((color_bytes & 0xF800) >> 11)?;
        let green_value = u8::try_from((color_bytes & 0x07E0) >> 5)?;
        let blue_value = u8::try_from(color_bytes & 0x001F)?;

        let r = red_value << 3;
        let g = green_value << 2;
        let b = blue_value << 3;

        Ok(Rgb565(r, g, b))
    }
}

impl Rgb565 {
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
