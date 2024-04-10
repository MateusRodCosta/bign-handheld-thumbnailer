pub struct Bgr555(u8, u8, u8);

impl TryFrom<u16> for Bgr555 {
    type Error = Box<dyn std::error::Error>;

    fn try_from(color_bytes: u16) -> Result<Bgr555, Self::Error> {
        /*
         * The NDS palette uses BGR555 for color encoding but we need RGB888
         * So, each individual color must be isolated and converted to RGB888
         */

        // Conversion code borrowed from
        // https://learn.microsoft.com/en-us/windows/win32/directshow/working-with-16-bit-rgb
        // with added swapping between Blue and Red

        let blue_value = u8::try_from((color_bytes & 0x7C00) >> 10)?;
        let green_value = u8::try_from((color_bytes & 0x03E0) >> 5)?;
        let red_value = u8::try_from(color_bytes & 0x001F)?;

        let b = blue_value << 3;
        let g = green_value << 3;
        let r = red_value << 3;

        Ok(Bgr555(r, g, b))
    }
}

impl Bgr555 {
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
